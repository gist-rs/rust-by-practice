const get_answers = (current) => {
  let _current = current
  let answers = []
  while (_current.nextSibling && _current.nextSibling.nextSibling && _current.nextSibling.nextSibling.nodeName === 'PRE' && _current.nextSibling.nextSibling.firstChild.childNodes[1].className.indexOf('answer')) {
    answers.push((_current = _current.nextSibling.nextSibling))
  }

  return answers
}

window.onload = function () {
  const dmp = new diff_match_patch()

  window.editors.forEach((editor, i) => {
    // keep raw
    editor.raw = editor.session.getValue()

    // Clear marker on focus
    function removeAllMarker(editor) {
      const prevMarkers = editor.session.getMarkers()
      if (prevMarkers) {
        const prevMarkersArr = Object.keys(prevMarkers)
        for (let item of prevMarkersArr) {
          editor.session.removeMarker(prevMarkers[item].id)
        }
      }
    }

    editor.on('change', () => removeAllMarker(editor))

    // Find answers after editor
    let current = editor.container.parentElement.parentNode
    editor.answers = get_answers(current)

    // Create buttons from answers
    let buttons = []
    editor.answers.forEach((answer, i) => {
      let button = document.createElement('button')
      let button_text = `🦀 HINT` + (editor.answers.length === 1 ? `` : ` ${i + 1}`)
      button.append(button_text)
      button.className = 'hint'
      button.alt = button_text
      let uncompleted_text = editor.raw

      button.onclick = () => {
        // Revet
        if (button.innerText.indexOf('🦀 HINT') === 0) {
          button.innerText = `✨ UNDO`
        } else {
          button.innerText = button.alt
          editor.setValue(uncompleted_text)
          editor.selection.selectTo(0)
          return
        }

        let answer_text = answer.firstChild.textContent

        // Patch first comment to make hint easy to focus
        if (uncompleted_text.indexOf('//') === 0 || uncompleted_text.indexOf('\n//') === 0) {
          const comments = uncompleted_text.split('\n').filter((e) => e.indexOf('//') === 0)
          answer_text = comments.join('\n') + '\n' + answer_text
        }
        
        // Always add newline
        answer_text = '\n' + answer_text

        editor.setValue(answer_text)
        editor.selection.selectTo(0)

        // Highlight patched
        const diffs = dmp.diff_main(uncompleted_text, answer_text)

        if (diffs.length >= 0) {
          let col = 0
          let row = 0
          diffs.forEach((diff) => {
            let patch_type = diff['0']
            let current_text = diff['1']
            let hunks = current_text.split('\n')
            let last_hunk = hunks[hunks.length - 1]
            let last_hunk_padded = last_hunk.split('\t').join('')

            const newline = Math.max(hunks.length - 1, 0)
            let row0 = row
            let column0 = col
            let row1 = row + newline
            let column1 = col + last_hunk_padded.length

            if (patch_type === 1) {
              let range = new ace.Range(row0, column0, row1, column1)
              // skip comment
              if (current_text.indexOf('//') !== 0) {
                editor.session.addMarker(range, 'ace_step', 'line', false)
              }

              // Reset column
              if (hunks.length > 1) {
                col = 0
              }

              // Bump
              col = col + last_hunk_padded.length
              row = row + newline
            } else if (patch_type === -1) {
              // Do nothing
            } else if (patch_type === 0) {
              // Reset column
              if (hunks.length > 1) {
                col = 0
              }

              // Bump
              col = col + last_hunk_padded.length
              row = row + newline
            }
          })
        }
      }

      buttons.push(button)
      answer.remove()
    })

    buttons.forEach((button) => current.append(button))
  })
}
