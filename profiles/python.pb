{% include "default" %}
(allow file-read* (literal "{{ home }}/.gitconfig"))
(allow file-read* (subpath "{{ home }}/.config/git"))
(allow file-read* (subpath "{{ home }}/.config/pip"))
(allow file-read* (subpath "{{ home }}/.pyenv"))
(allow file-read* (subpath "{{ home }}/.local/lib"))
(allow file-read* (literal "{{ home }}/.python_history"))
(allow file-write* (subpath "{{ home }}/Library/Caches"))
(allow file-write* (literal "{{ home }}/.python_history"))
