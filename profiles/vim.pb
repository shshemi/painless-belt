{% include "default" %}
(allow file-read* (literal "{{ home }}/.vimrc"))
(allow file-read* (subpath "{{ home }}/.vim"))
(allow file-write* (subpath "{{ home }}/.vim"))
(allow file-read* (literal "{{ home }}/.viminfo"))
(allow file-write* (literal "{{ home }}/.viminfo"))
