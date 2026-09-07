import {
  Icon,
  ToastViewport,
  type Tint,
  useToast,
} from '@dazzlabs/dazzui-react'
import { CheckmarkCircle20Regular } from '@fluentui/react-icons'
import { useEffect, useMemo, useRef, useState } from 'react'

import { CanvasStage } from './components/canvas-stage'
import { DocumentSidebar } from './components/document-sidebar'
import { ExportDialog } from './components/export-dialog'
import { InspectAside } from './components/inspect-aside'
import { ToolRail } from './components/tool-rail'
import { ViewBar } from './components/view-bar'
import { WindowFrame } from './components/window-frame'
import { DOCUMENT } from './data/document'
import { TOOLS, type ToolId } from './data/tools'
import { union } from './lib/geometry'
import { isTyping } from './lib/keys'
import {
  align,
  copies,
  type Edge,
  GRID,
  insert,
  patch,
  remove,
  reorder,
  type Shape,
  useDocument,
} from './lib/scene'
import { applyTheme, type Theme } from './lib/theme'
import { nextZoom, useViewport } from './lib/viewport'

export function App() {
  const doc = useDocument(DOCUMENT)
  const view = useViewport()

  const [theme, setTheme] = useState<Theme>('studio-light')
  const [tool, setTool] = useState<ToolId>('select')
  const [tint, setTint] = useState<Tint>('primary')
  const [grid, setGrid] = useState(true)
  const [snapping, setSnapping] = useState(true)
  const [sidebarOpen, setSidebarOpen] = useState(true)
  const [exporting, setExporting] = useState(false)
  const toast = useToast()

  const { page, pageId, selection, selected } = doc
  const { fit, size, zoomBy, zoomTo } = view

  const content = useMemo(() => union(page.shapes), [page.shapes])
  const box = union(selected) ?? content

  /* Opening a page frames what is on it — including the page opened first,
     as soon as the stage has measured itself and there is a stage to fit to. */
  const framed = useRef<string | null>(null)
  useEffect(() => {
    if (size.width === 0) return
    if (framed.current === pageId) return
    framed.current = pageId
    fit(union(page.shapes))
  }, [fit, page.shapes, pageId, size.width])

  const chooseTheme = (next: Theme) => {
    applyTheme(next)
    setTheme(next)
  }

  const patchSelection = (change: Partial<Shape>) => {
    if (selection.length === 0) return
    doc.edit(patch(selection, change))
  }

  const nudge = (dx: number, dy: number) => {
    if (selection.length === 0) return
    doc.edit(patch(selection, shape => ({ x: shape.x + dx, y: shape.y + dy })))
  }

  const duplicate = () => {
    if (selection.length === 0) return
    const made = copies(page, selection)
    doc.edit(insert(made))
    doc.select(made.map(shape => shape.id))
  }

  const deleteSelection = () => {
    if (selection.length === 0) return
    doc.edit(remove(selection))
    doc.select([])
  }

  const selectAll = () =>
    doc.select(
      page.shapes
        .filter(shape => !shape.hidden && !shape.locked)
        .map(shape => shape.id)
    )

  /*
   * The keyboard.
   *
   * It is bound on the window rather than on the canvas, because the canvas is
   * not focusable and the keys are the app's rather than one column's — but it
   * steps aside for anything being typed into, which is what `isTyping` is
   * for: a bare `R` draws a rectangle everywhere except inside a field.
   */
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (isTyping(event.target)) return
      const mod = event.metaKey || event.ctrlKey
      const step = event.shiftKey ? GRID : 1

      if (mod && event.key.toLowerCase() === 'z') {
        event.preventDefault()
        if (event.shiftKey) doc.redo()
        else doc.undo()
        return
      }
      if (mod && event.key.toLowerCase() === 'd') {
        event.preventDefault()
        duplicate()
        return
      }
      if (mod && event.key.toLowerCase() === 'a') {
        event.preventDefault()
        selectAll()
        return
      }
      if (mod && event.key === '0') {
        event.preventDefault()
        zoomTo(1)
        return
      }
      if (mod && (event.key === '=' || event.key === '+')) {
        event.preventDefault()
        zoomTo(nextZoom(view.viewport.zoom, 1))
        return
      }
      if (mod && event.key === '-') {
        event.preventDefault()
        zoomTo(nextZoom(view.viewport.zoom, -1))
        return
      }
      if (mod) return

      switch (event.key) {
        case 'Backspace':
        case 'Delete':
          event.preventDefault()
          deleteSelection()
          return
        case 'Escape':
          if (exporting) setExporting(false)
          else doc.select([])
          return
        case '!':
          // Shift+1, whatever the layout calls it.
          fit(union(page.shapes))
          return
        case 'ArrowUp':
          event.preventDefault()
          nudge(0, -step)
          return
        case 'ArrowDown':
          event.preventDefault()
          nudge(0, step)
          return
        case 'ArrowLeft':
          event.preventDefault()
          nudge(-step, 0)
          return
        case 'ArrowRight':
          event.preventDefault()
          nudge(step, 0)
          return
      }

      const picked = TOOLS.find(
        entry => entry.key.toLowerCase() === event.key.toLowerCase()
      )
      if (picked) setTool(picked.id)
    }

    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
    // Deliberately re-bound on every render rather than pinned to a dependency
    // list: the handler reads the selection, the page and the zoom, and a
    // listener that closed over last render's copy of those would act on a
    // drawing that has moved on.
  })

  return (
    <div className="cv-root">
      <WindowFrame
        canRedo={doc.canRedo}
        canUndo={doc.canUndo}
        name={doc.name}
        page={page.name}
        sidebar={
          <DocumentSidebar
            page={page}
            pageId={pageId}
            pages={doc.pages}
            selection={selection}
            onAddPage={doc.addPage}
            onOpenPage={doc.openPage}
            onSelect={(id, additive) =>
              doc.select(
                additive
                  ? selection.includes(id)
                    ? selection.filter(entry => entry !== id)
                    : [...selection, id]
                  : [id]
              )
            }
            onSetHidden={(id, hidden) => doc.edit(patch([id], { hidden }))}
            onSetLocked={(id, locked) => doc.edit(patch([id], { locked }))}
          />
        }
        sidebarOpen={sidebarOpen}
        theme={theme}
        onExport={() => setExporting(true)}
        onRedo={doc.redo}
        onSidebarOpenChange={setSidebarOpen}
        onThemeChange={chooseTheme}
        onUndo={doc.undo}
      >
        <ToolRail tool={tool} onPick={setTool} />

        <main className="cv-main">
          <CanvasStage
            doc={doc}
            grid={grid}
            page={page}
            selection={selection}
            snapping={snapping}
            tint={tint}
            tool={tool}
            view={view}
            onPickTool={setTool}
          />

          <ViewBar
            grid={grid}
            selected={selection.length}
            snapping={snapping}
            total={page.shapes.length}
            zoom={view.viewport.zoom}
            onFit={() => fit(content)}
            onGridChange={setGrid}
            onSnappingChange={setSnapping}
            onZoomBy={factor => zoomBy(factor)}
            onZoomTo={zoomTo}
          />
        </main>

        <InspectAside
          page={page}
          selected={selected}
          tint={tint}
          typing={{
            start: doc.begin,
            draft: change => doc.draft(patch(selection, change)),
            end: doc.settle,
          }}
          onAlign={(edge: Edge) => doc.edit(align(selection, edge))}
          onDelete={deleteSelection}
          onDuplicate={duplicate}
          onOrder={direction => doc.edit(reorder(selection, direction))}
          onPatch={patchSelection}
          onTintChange={setTint}
        />
      </WindowFrame>

      {exporting ? (
        <ExportDialog
          box={box}
          count={selection.length || page.shapes.length}
          whole={selection.length === 0}
          onClose={() => setExporting(false)}
          onExport={summary => {
            setExporting(false)
            toast.add({
              title: `${summary} — nothing was written.`,
              tint: 'success',
              icon: <Icon icon={CheckmarkCircle20Regular} size={16} />,
            })
          }}
        />
      ) : null}

      <ToastViewport />
    </div>
  )
}
