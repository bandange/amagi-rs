(function () {
  var CODE_LANGUAGE_ALIASES = {
    dotenv: "ini",
    env: "ini",
    powershell: "plaintext",
    ps1: "plaintext",
    pwsh: "plaintext",
  };

  function codeBlockScope(root) {
    return root && root.querySelectorAll ? root : document;
  }

  function codeBlockLanguage(block) {
    for (var index = 0; index < block.classList.length; index += 1) {
      var className = block.classList[index];

      if (className.indexOf("language-") === 0) {
        return className.slice("language-".length).toLowerCase();
      }
    }

    return "";
  }

  function normalizeCodeBlockLanguage(block) {
    if (!window.hljs || !window.hljs.getLanguage) {
      return;
    }

    var requestedLanguage = codeBlockLanguage(block);
    if (!requestedLanguage) {
      return;
    }

    var highlightLanguage =
      CODE_LANGUAGE_ALIASES[requestedLanguage] || requestedLanguage;

    if (!window.hljs.getLanguage(highlightLanguage)) {
      highlightLanguage = "plaintext";
    }

    if (highlightLanguage !== requestedLanguage) {
      block.classList.remove("language-" + requestedLanguage);
      block.classList.add("language-" + highlightLanguage);
      block.dataset.originalLanguage = requestedLanguage;
    }
  }

  function fallbackCopy(text) {
    var input = document.createElement("textarea");
    input.value = text;
    input.setAttribute("readonly", "");
    input.style.position = "fixed";
    input.style.top = "-999px";
    input.style.opacity = "0";
    document.body.appendChild(input);
    input.select();

    try {
      return document.execCommand("copy");
    } finally {
      document.body.removeChild(input);
    }
  }

  function setCopyState(button, copied) {
    button.classList.toggle("copied", copied);
    button.setAttribute("aria-label", copied ? "Copied" : "Copy code");
    button.setAttribute("title", copied ? "Copied" : "Copy code");
    button.dataset.label = copied ? "Copied" : "Copy";
  }

  function copyCodeBlock(pre, button) {
    var code = pre.querySelector("code");
    var text = code ? code.textContent : pre.textContent;
    var copied = false;

    if (navigator.clipboard && window.isSecureContext) {
      navigator.clipboard
        .writeText(text)
        .then(function () {
          setCopyState(button, true);
          window.setTimeout(function () {
            setCopyState(button, false);
          }, 1500);
        })
        .catch(function () {
          if (fallbackCopy(text)) {
            setCopyState(button, true);
            window.setTimeout(function () {
              setCopyState(button, false);
            }, 1500);
          }
        });
      return;
    }

    copied = fallbackCopy(text);
    if (copied) {
      setCopyState(button, true);
      window.setTimeout(function () {
        setCopyState(button, false);
      }, 1500);
    }
  }

  function enhanceCodeBlocks(root) {
    var scope = codeBlockScope(root);
    scope
      .querySelectorAll(".markdown-body pre:not([data-copy-enhanced])")
      .forEach(function (pre) {
        if (!pre.querySelector("code") || pre.closest(".code-block-shell")) {
          pre.dataset.copyEnhanced = "true";
          return;
        }

        var shell = document.createElement("div");
        var button = document.createElement("button");

        shell.className = "code-block-shell";
        button.className = "code-copy-button";
        button.type = "button";
        button.innerHTML = '<span class="code-copy-icon" aria-hidden="true"></span>';
        setCopyState(button, false);
        button.addEventListener("click", function () {
          copyCodeBlock(pre, button);
        });

        pre.dataset.copyEnhanced = "true";
        pre.parentNode.insertBefore(shell, pre);
        shell.appendChild(pre);
        shell.appendChild(button);
      });
  }

  function highlightCodeBlocks(root) {
    if (!window.hljs) {
      return;
    }

    var scope = codeBlockScope(root);
    scope
      .querySelectorAll(".markdown-body pre code:not([data-highlighted])")
      .forEach(function (block) {
        normalizeCodeBlockLanguage(block);
        window.hljs.highlightElement(block);
      });
  }

  function processCodeBlocks(root) {
    enhanceCodeBlocks(root);
    highlightCodeBlocks(root);
  }

  window.amagiHighlightCode = function (root) {
    processCodeBlocks(root || document);
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () {
      processCodeBlocks(document);
    });
  } else {
    processCodeBlocks(document);
  }

  var observer = new MutationObserver(function (mutations) {
    mutations.forEach(function (mutation) {
      mutation.addedNodes.forEach(function (node) {
        if (node.nodeType === Node.ELEMENT_NODE) {
          processCodeBlocks(node);
        }
      });
    });
  });

  observer.observe(document.documentElement, {
    childList: true,
    subtree: true,
  });
})();
