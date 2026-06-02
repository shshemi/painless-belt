{% include "default" %}
(allow file-read* (literal "{{ home }}/.gitconfig"))
(allow file-read* (subpath "{{ home }}/.config/git"))
(allow file-read* (subpath "{{ home }}/.config/gh"))
(allow file-read* (subpath "{{ home }}/Library/Keychains"))
(allow file-write* (subpath "{{ home }}/.config/gh"))
(allow file-write* (subpath "{{ home }}/Library/Caches"))
