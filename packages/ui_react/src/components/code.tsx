import { Checkmark16Regular, Copy16Regular } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'
import { useEffect, useState } from 'react'

import { cx } from '../lib/cx'
import { IconButton } from './icon-button'

export interface CodeProps extends ComponentPropsWithoutRef<'code'> {}

/**
 * A run of code inside a sentence — a flag, a path, an identifier.
 *
 * It is a wash and a face, not a box: the chip corner and the tiny inset keep
 * it separable from the prose around it without breaking the line's rhythm,
 * which is what a bordered box at this size always does.
 */
export function Code({ className, ...props }: CodeProps) {
  return <code className={cx('dz-code', className)} {...props} />
}

export interface CodeBlockProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  /** The source, verbatim. Trailing newlines are trimmed. */
  code: string
  /** What the block is — a path, a filename, a shell's name. */
  title?: ReactNode
  /**
   * Named at the trailing edge of the head, and nothing more: this package
   * ships no highlighter, because a grammar per language is a dependency the
   * design system has no opinion about. An app that wants colour renders its
   * own runs as `children` of a `pre` instead.
   */
  language?: string
  lineNumbers?: boolean
  /** Wraps long lines instead of scrolling them. */
  wrap?: boolean
  /** Offers the whole block to the clipboard. */
  copyable?: boolean
  /** Extra affordances in the head, before the copy button. */
  actions?: ReactNode
}

/**
 * A block of code, read rather than edited.
 *
 * The gutter is a column of its own rather than text in the same run, so a
 * selection that crosses ten lines copies ten lines of code and not ten line
 * numbers — which is the single thing a reader does with one of these, and
 * the thing a numbered `<pre>` gets wrong.
 *
 * A head is drawn only where there is something to put in it. A block with no
 * title, no language and nothing to press is the code alone, because a bar
 * holding one label is furniture that says less than the space it takes.
 */
export function CodeBlock({
  code,
  title,
  language,
  lineNumbers = false,
  wrap = false,
  copyable = false,
  actions,
  className,
  ...props
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false)
  const body = code.replace(/\n+$/, '')
  const lines = body.split('\n')
  const head = title || language || actions || copyable

  /* The tick is a state with a lifetime, so it has to be cleared when the
     block unmounts between the copy and the timeout — otherwise React is set
     on an element that has gone. */
  useEffect(() => {
    if (!copied) return
    const timer = setTimeout(() => setCopied(false), 1600)
    return () => clearTimeout(timer)
  }, [copied])

  return (
    <div
      data-tint="neutral"
      data-wrap={wrap}
      className={cx('dz-code-block', className)}
      {...props}
    >
      {head ? (
        <div className="dz-code-block__head">
          {title ? <span className="dz-code-block__title">{title}</span> : null}
          {language ? (
            <span className="dz-code-block__language">{language}</span>
          ) : null}
          <span className="dz-code-block__actions">
            {actions}
            {copyable ? (
              <IconButton
                label={copied ? 'Copied' : 'Copy'}
                onClick={() => {
                  void navigator.clipboard?.writeText(body).then(
                    () => setCopied(true),
                    /* A clipboard the browser refused is not an error the
                       reader can act on, and a block that throws here would
                       take the page with it. The tick simply never appears. */
                    () => {}
                  )
                }}
              >
                {copied ? <Checkmark16Regular /> : <Copy16Regular />}
              </IconButton>
            ) : null}
          </span>
        </div>
      ) : null}
      <div className="dz-code-block__scroll">
        {lineNumbers ? (
          <div aria-hidden className="dz-code-block__gutter">
            {lines.map((_, index) => (
              // Line numbers have no identity of their own, and the list
              // never reorders.
              <span key={index}>{index + 1}</span>
            ))}
          </div>
        ) : null}
        <pre className="dz-code-block__pre">
          <code>{body}</code>
        </pre>
      </div>
    </div>
  )
}
