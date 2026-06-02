{% include "default" %}
(allow file-read* (literal "{{ home }}/.gitconfig"))
(allow file-read* (subpath "{{ home }}/.config/git"))
(allow file-read* (subpath "{{ home }}/go"))
(allow file-write* (subpath "{{ home }}/go"))
(allow file-write* (subpath "{{ home }}/Library/Caches"))
