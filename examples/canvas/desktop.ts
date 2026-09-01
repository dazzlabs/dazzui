import { serveDir } from '@std/http/file-server'

/**
 * The window the app opens at.
 *
 * Not a token: a design system decides what goes inside a window, not how big
 * the window is. A canvas wants more of it than most apps do — three chrome
 * columns stand either side of the drawing, and what is left over is the
 * drawing — so this one opens wider than the reading apps beside it.
 *
 * The titlebar is transparent rather than drawn, because the app already has
 * one: its toolbar sits at `--frame-titlebar-size` and the window controls
 * land in the sidebar's header strip beside it.
 */
const WINDOW: Deno.BrowserWindowOptions = {
  // Empty rather than absent: with the titlebar hidden a title would be drawn
  // over the toolbar, which already carries one of its own. The app is still
  // called Canvas everywhere the system names it — that comes from
  // `desktop.app.name`, not from here.
  title: '',
  width: 1360,
  height: 900,
  transparentTitlebar: true,
}

/**
 * The Vite build, resolved against this module rather than the working
 * directory: in a packaged app these files live in the binary's virtual file
 * system, and `Deno.cwd()` is wherever the user happened to launch from.
 */
const ROOT = `${import.meta.dirname}/dist`

/**
 * The document, marked with which shell it is in and stripped of its title.
 *
 * The page reserves room for window controls, which is right in a window that
 * has them and a gap in a browser tab. One attribute is enough for the
 * stylesheet to tell the two apart, and it beats shipping a second build.
 *
 * The title goes for the same reason the window's did: a webview hands the
 * document's title to the window it is in, which would put one straight back
 * over a toolbar that already carries its own. A browser tab still wants a
 * name, so it is emptied here rather than in `index.html`.
 */
const DOCUMENT = (await Deno.readTextFile(`${ROOT}/index.html`))
  .replace('<html ', '<html data-shell="desktop" ')
  .replace(/<title>[^<]*<\/title>/, '<title></title>')

new Deno.BrowserWindow(WINDOW)

// No port and no hostname: the webview navigates to whatever this binds to.
Deno.serve(async request => {
  const { pathname } = new URL(request.url)

  // Everything Vite emits carries an extension, so a path without one is a
  // route rather than a file — hand back the document and let the client read
  // the URL.
  if (!/\.[a-z0-9]+$/i.test(pathname)) {
    return new Response(DOCUMENT, {
      headers: { 'content-type': 'text/html; charset=utf-8' },
    })
  }

  return await serveDir(request, { fsRoot: ROOT, quiet: true })
})
