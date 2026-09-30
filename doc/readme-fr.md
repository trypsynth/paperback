<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

# Paperback - version 1.0

## Introduction

Paperback est un lecteur léger, rapide et accessible pour les livres électroniques, les documents et les livres audio, pour tout le monde, des lecteurs occasionnels aux utilisateurs avancés. Il est conçu pour l'accessibilité aux lecteurs d'écran, des vitesses rapides et une expérience sans surcharge.

## Configuration requise

Paperback fonctionne sur Windows 10/11, toutes les versions modernes d'ARM macOS, Linux, iOS 17 et ultérieur, et Android 7 et ultérieur. Les applications iOS et Android sont disponibles sur l'App Store et Google Play.

## Fonctionnalités

* Complètement autonome, ne nécessitant aucun logiciel à installer sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans chaque document que vous ouvrez.
* Se souvient optionnellement des documents que vous aviez ouverts à la fermeture du programme, et les restaure au prochain lancement.
* Inclut une fonctionnalité de navigation similaire à celle trouvée dans le mode de navigation Web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut une boîte de dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et le support des expressions régulières.
* Peut être exécuté entièrement de manière portable, ou installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un large éventail de formats de fichiers courants.
* Lit les livres audio, avec une vitesse ajustable et des signets qui mémorisent le temps exact.
* Lit les pages PDF numérisées avec l'OCR intégré à Windows et macOS.
* Signets et notes, pour que vous puissiez marquer votre place et y revenir.
* Chaque raccourci clavier peut être modifié.
* Livré avec `pb`, un outil en ligne de commande qui convertit tout document pris en charge en HTML, Markdown ou texte brut.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il existe cependant deux problèmes connus pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous constaterez peut-être que les longs paragraphes sont tronqués lors du panoramique vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également affectée. Il s'agit d'un bug dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non dans Paperback lui-même, et il a fallu un certain temps pour trouver un correctif compte tenu de l'enthousiasme de Vispero à répondre aux problèmes des logiciels open source.

La solution de contournement, finalement mise au jour par le groupe de discussion JAWS après des mois d'attente, consiste à modifier `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif au lieu d'avancer. Avec ces deux paramètres en place, le panoramique devrait fonctionner correctement.

### JAWS et les messages de Paperback

Paperback dit des choses comme « No pages. » ou « This document has no audio. » en tant que notifications d'accessibilité, ce qui permet à un lecteur d'écran de les lire par rapport à ce qu'il dit. JAWS n'agit sur ces notifications que lorsque « Enable accessible notification events » est activé pour l'application, et sur certaines machines, ce n'est pas le cas.

Si JAWS ne dit rien lorsque vous appuyez sur une touche qui devrait signaler quelque chose, ouvrez le Centre des paramètres avec Paperback au premier plan (`Insert+6`), recherchez « notification » et cochez « Enable accessible notification events ». Cela écrit le paramètre dans `paperback.jcf`, de sorte qu'il s'applique à Paperback seul.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Archives de bandes dessinées (`.cbz`)
* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Livres électroniques FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Pages de manuel, à la fois `man` et BSD `mdoc` (`.1` à `.9`, `.man`, `.roff` et les versions compressées de chacun)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres audio M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Livres audio MP3 (`.mp3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents reStructuredText (`.rst`, `.rest`)
* Documents RTF (`.rtf`)
* Documents Windows Write (`.wri`)
* Fichiers WinHelp (`.hlp`)
* Fichiers texte brut et fichiers journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation en priorité au clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous concernent Windows. Lorsque macOS diffère, l'équivalent est indiqué entre parenthèses — principalement parce que Ctrl+G, Ctrl+W et Alt+Left/Right sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actif.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des Documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, cette option se trouve dans le menu de l'application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher le suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher le précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (si supporté par le document actif).
* `=` : Annoncer votre pourcentage de lecture actuel et votre page, par exemple « 15%, page 30 ». La page est omise pour les documents sans numéros de page.
* `Alt+Left` (macOS : `Cmd+[`) : Revenir en arrière dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l'historique de navigation.
* `[` : Section précédente.
* `]` : Section suivante.
* `Shift+H` : Titre précédent.
* `H` : Titre suivant.
* `Shift+1` à `Shift+6` : Titre précédent de niveau 1-6.
* `1` à `6` : Titre suivant de niveau 1-6.
* `Shift+P` : Page précédente.
* `P` : Page suivante.
* `Shift+B` : Signet précédent.
* `B` : Signet suivant.
* `/` : Définir votre signet temporaire.
* `\` : Accéder à votre signet temporaire.
* `Shift+N` : Note précédente.
* `N` : Note suivante.
* `Ctrl+B` : Aller à tous les signets et notes.
* `Ctrl+Alt+B` : Aller aux signets uniquement.
* `Ctrl+Alt+M` : Aller aux notes uniquement.
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le texte de la note à la position actuelle.
* `Shift+K` : Lien précédent.
* `K` : Lien suivant.
* `Shift+G` : Image précédente.
* `G` : Image suivante.
* `Shift+F` : Figure précédente.
* `F` : Figure suivante.
* `Shift+T` : Tableau précédent.
* `T` : Tableau suivant.
* `Shift+M` : Formule précédente.
* `M` : Formule suivante.
* `Shift+S` : Séparateur précédent.
* `S` : Séparateur suivant.
* `Shift+L` : Liste précédente.
* `L` : Liste suivante.
* `Shift+I` : Élément de liste précédent.
* `I` : Élément de liste suivant.
* `Shift+,` : Aller au début du conteneur actif (liste ou tableau).
* `,` : Dépasser la fin du conteneur actif (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document actif.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu actuel dans la Vue Web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document actif en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/position du curseur actuelle.
* `Ctrl+Shift+N` : Ajouter ou éditer une note de signet à la sélection/position du curseur actuelle.
* `Ctrl+Alt+W` : Basculer le retour à la ligne automatique.
* `Ctrl+Space` (macOS : `RawCtrl+Space`, c'est-à-dire la touche Control physique, car Cmd+Space ouvre Spotlight) : Lecture/pause de la narration audio.
* `'` : Avancer rapidement dans la narration audio.
* `;` : Reculer rapidement dans la narration audio.
* `Shift+'` : Augmenter le montant de l'avance rapide audio.
* `Shift+;` : Diminuer le montant de l'avance rapide audio.
* `Ctrl+Shift+.` : Accélérer la narration audio.
* `Ctrl+Shift+,` : Ralentir la narration audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le mode plein écran.
* `Ctrl+,` : Ouvrir les Paramètres (macOS : dans le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de sommeil.
* `Ctrl+Shift+O` : Reconnaître une plage de pages PDF numérisées avec OCR.
* `Alt+F9` (macOS : `Cmd+F9`) : Marquer le début d'une sélection, de sorte que tout ce qui va d'ici à votre position actuelle puisse être copié en une seule fois.
* `Alt+F10` (macOS : `Cmd+F10`) : Copier tout du début marqué de la sélection à la position actuelle.
* `Alt+Shift+F9` (macOS : `Cmd+Shift+F9`) : Revenir au début marqué de la sélection, en laissant la marque en place.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de don dans votre navigateur par défaut.

### Touches supplémentaires de la vue des documents

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet de document sélectionné.
* `Enter` ou `Space` dans le texte du document : Suivre un lien ou ouvrir une vue de tableau ou de formule au curseur.
* `Enter` sur une page PDF numérisée : Reconnaître la page avec OCR.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## iOS et Android

Les applications iOS et Android utilisent le même moteur de lecture que le bureau, ils ouvrent donc les mêmes formats et se souviennent de votre place de la même manière. Elles sont conçues pour être utilisées avec VoiceOver sur iOS et TalkBack sur Android.

### Ouverture de documents

* Utilisez le bouton Open Book, ou ouvrez un document depuis l'application Files ou une autre application et choisissez Paperback.
* Sur Android, vous pouvez activer le navigateur de fichiers intégré dans les Paramètres à la place. Il a besoin de la permission d'accès à tous les fichiers, et ouvre les gros fichiers directement au lieu de les copier d'abord.
* Appuyez longuement sur le bouton Open Book pour importer ou exporter les données d'un document (`.paperback`), les mêmes fichiers que l'application de bureau utilise.

### Lecture et écoute

Chaque application a deux façons de lire un document. En mode texte, vous lisez le texte avec votre lecteur d'écran. En mode lecture à haute voix, Paperback lit le texte pour vous avec la voix que vous choisissez dans les Paramètres, et continue en arrière-plan et depuis l'écran de verrouillage. Basculez entre les deux depuis le menu Plus d'options.

Les livres audio, comme DAISY, M4B et les livres MP3, lisent leur propre enregistrement à la place.

### La barre de lecture

La barre en bas de l'écran a, de gauche à droite :

* L'unité de navigation, comme paragraphe, titre, page ou lien. Balayez vers le haut ou vers le bas pour la changer.
* Les boutons Précédent, Lecture et Suivant. Précédent et Suivant se déplacent par l'unité de navigation.
* La vitesse de parole. Balayez vers le haut ou vers le bas pour changer la vitesse de lecture de Paperback.

Vous pouvez également balayer vers le haut ou vers le bas sur le bouton de lecture pour vous déplacer par l'unité de navigation, sans avoir besoin d'atteindre les boutons Précédent et Suivant. Si c'est tout ce que vous utilisez, le paramètre Masquer les boutons Précédent et Suivant les retire du chemin de votre lecteur d'écran. Le paramètre Balayage vers le haut avance choisit la direction dans laquelle va un balayage.

### Plus d'options

Le menu Plus d'options est l'endroit où tout le reste se trouve. Certains éléments fonctionnent un peu différemment sur chaque application.

* **Passer au mode TTS ou Passer au mode Texte :** bascule entre le mode lecture à haute voix et le mode texte, décrit ci-dessus. En mode texte, un élément Lire à haute voix lance et met en pause la lecture à haute voix sans quitter le mode texte.
* **Table des matières :** les chapitres du livre, ouverts à celui que vous lisez. Choisissez-en un pour y aller directement. Les entrées ayant des chapitres sous elles peuvent être développées et réduites avec les actions du lecteur d'écran.
* **Éléments :** une liste des titres ou des liens du document. Basculez entre les deux avec le sélecteur Type sur iOS, ou les onglets sur Android, puis choisissez-en un pour y aller.
* **Rechercher :** tapez ce que vous cherchez, ou choisissez une recherche antérieure dans l'Historique de recherche, et choisissez s'il faut respecter la casse, faire correspondre des mots entiers seulement, ou utiliser une expression régulière. Rechercher le précédent et Rechercher le suivant sautent à une correspondance et indiquent où c'est tombé, et Rechercher reste ouvert pour que vous puissiez continuer. En mode lecture à haute voix, Rechercher s'affiche également comme une unité de navigation sur la barre de lecture, ce qui vous permet de parcourir les correspondances à partir de là aussi.
* **Aller à :** sautez à une ligne, une page, ou un pourcentage du document. Choisissez lequel avec le sélecteur Mode.
* **Documents récents :** tous les documents que vous avez ouverts, chacun marqué comme actuellement ouvert, fermé ou fichier manquant. Chacun a deux actions du lecteur d'écran : Supprimer l'enlève de la liste, et Localiser vous permet de trouver un document dont le fichier a été déplacé. Effacer les documents récents vide la liste sans supprimer de documents.
* **Nombre de mots :** le nombre de mots du document.
* **Informations sur le document :** le titre, l'auteur, le nom du fichier, et sur iOS aussi le nombre de lignes et de caractères.
* **Exporter :** enregistre le document en texte brut, HTML ou Markdown.
* **Minuteur de sommeil :** arrête la lecture après 5, 10, 15, 30, 45 ou 60 minutes, ou une durée de votre choix. Ouvrez-le à nouveau pendant qu'il fonctionne pour voir combien de temps il reste, ou pour l'annuler.
* **Aide :** ouvre ce fichier readme.
* **Paramètres :**
    * **Synthèse vocale :** la voix, la vitesse de parole et la hauteur, un bouton Lire un exemple pour les entendre, et la pause entre les paragraphes. Android vous permet également de choisir le moteur de synthèse vocale. Sur iOS, c'est aussi là que se trouve le dictionnaire de parole : des règles qui modifient la façon dont les mots sont prononcés, pour toutes les voix ou juste certaines.
    * **Lisibilité :** taille du texte, interligne, espacement des paragraphes, alignement et texte à contraste élevé. iOS a aussi une apparence claire et sombre.
    * **Comportement :** s'il faut rouvrir vos documents au démarrage de l'application, la direction dans laquelle un balayage sur le bouton de lecture se déplace, et s'il faut masquer les boutons Précédent et Suivant. Android a aussi le navigateur de fichiers intégré ici.

### Claviers et casques

Avec un clavier, les raccourcis de bureau pour ouvrir des livres, les documents récents, Rechercher, Aller à, la table des matières, le nombre de mots, les informations sur le document, l'exportation et la minuterie de sommeil fonctionnent tous, en utilisant `Cmd` à la place de `Ctrl` sur iOS. Tout comme les touches à une seule lettre pour se déplacer par titre, page, lien et le reste, et `Space` pour jouer et mettre en pause. Sur iOS, les touches à une seule lettre ne touchent Paperback que lorsque la navigation rapide à une lettre VoiceOver est désactivée.

Sur Android, un bouton de casque joue et met en pause avec une pression, avance avec deux, et revient avec trois.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, avec d'autres qui s'ajoutent continuellement. Une liste complète suit ci-dessous.

Pour apprendre comment contribuer, veuillez lire notre [Guide de traduction](translating.md).

* Bosniaque
* Tchèque
* Néerlandais
* Finnois
* Français
* Allemand
* Japonais
* Polonais
* Portugais (Brésil)
* Russe
* Chinois simplifié
* Serbe
* Espagnol
* Ukrainien
* Vietnamien

## Crédits
### Développement
* Quin Gillespie : développeur principal et fondateur du projet.
* Aryan Choudhary : contributeur principal.

### Donations
Les personnes suivantes ont fait des dons de diverses montants au développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici, je n'ajoute que les personnes qui souhaitent que leur don soit rendu public.

Remarque : je considère un parrain GitHub public comme un motif d'inclusion automatique dans cette liste.

* Alex Hall
* Brandon McGinty
* Brian Hartgen
* Debbie Yuille
* Devin Prater
* Felix Steindorff
* Hamish Mackenzie
* James Scholes
* Jayson Smith
* Jonathan Rodriguez
* Jonathan Schuster
* Keao Wright
* Michael Marshall
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Journal des modifications

### Version 1.0

1.0 est la première version sur les cinq plates-formes : Windows, macOS, Linux, iOS et Android, avec les applications iOS et Android disponibles sur l'App Store et Google Play.

#### Ajouts

##### Général
* Support de Linux, en tant qu'AppImage ou tar.gz, avec intégration au bureau de sorte que les documents s'ouvrent à partir de votre gestionnaire de fichiers.
* Marquer le début d'une sélection avec `Alt+F9`, copier tout ce qui s'y trouve jusqu'à votre position actuelle avec `Alt+F10`, et revenir à la marque avec `Alt+Shift+F9`, pour copier un long passage de texte sans utiliser shift+flèche. Les trois options se trouvent sous Outils > Sélectionner et copier.
* Le raccourci `=` annonce maintenant la page ainsi que le pourcentage, par exemple « 15 %, page 30 », et reste inchangé pour les documents sans numérotation de page.
* La boîte À propos affiche maintenant la licence de Paperback et tous les traducteurs.
* Une traduction en ukrainien.

##### Nouveaux formats
* Archives de bandes dessinées (`.cbz`).
* Audiolivres M4B, divisés en leurs chapitres.
* Pages de manuel, à la fois `man` et BSD `mdoc`, compressées ou non.
* Audiolivres MP3, divisés en chapitres si le fichier les contient.
* Documents reStructuredText.
* Fichiers Windows Write (`.wri`).
* Fichiers WinHelp (`.hlp`).
* Documents Word 6 et Word 95.

##### OCR
* Les pages PDF numérisées peuvent maintenant être reconnues avec l'OCR intégré à Windows et macOS. Appuyez sur `Enter` sur une page numérisée pour la reconnaître, ou utilisez l'OCR par lot (`Ctrl+Shift+O`) pour une série de pages.

##### Navigation
* Les formules MathML dans EPUB et HTML sont rendues sous forme d'AsciiMath en utilisant MathCAT. Utilisez `M` ou `Shift+M` pour naviguer dans les formules, puis `Enter` ou `Space` pour ouvrir le MathML original dans la vue Formule.
* Un bouton Rechercher tout dans la boîte de dialogue Rechercher, listant chaque ligne contenant une correspondance pour que vous puissiez accéder directement à celle que vous voulez.
* Vues Tableaux, Listes et Pages dans la liste des éléments (`F7`).
* Aller à la ligne, Aller à la page et Aller au pourcentage acceptent maintenant `+n` et `-n` pour vous déplacer par rapport à votre position actuelle.
* Les livres EPUB, MOBI et CHM sans leurs propres en-têtes disposent maintenant de la navigation par en-têtes provenant de leur table des matières.
* Les livres KF8 (AZW3) prennent maintenant en charge la navigation par sections.
* Les pages EPUB qui ne sont qu'une image affichent maintenant une ligne pour celle-ci, afin que vous puissiez vous y arrêter au lieu de les dépasser directement.

##### Audiolivres
* Contrôles de vitesse de lecture, de la demi-vitesse à trois fois plus rapide. Utilisez `Ctrl+Shift+.` et `Ctrl+Shift+,`, ou le menu Outils.
* Les signets et notes dans les livres audio uniquement mémorisent maintenant l'heure exacte à laquelle vous les avez définis.
* Position suivante et précédente (`Alt+Left` et `Alt+Right`) fonctionnent maintenant dans les audiolivres.
* La progression dans un audiolivre est maintenant mesurée par son enregistrement, de sorte que Aller au pourcentage et la barre d'état correspondent à votre progression réelle.

##### Documents récents
* Un élément Effacer les documents récents dans le sous-menu Documents récents.

##### Documents PDF
* Un paramètre pour conserver chaque ligne d'un PDF séparée, plutôt que de les fusionner en paragraphes.
* Les images et figures dans les PDF sont maintenant annoncées.
* Les PDF qui portent une structure de lecture mais ne balisent aucune de leurs images annoncent maintenant ces images, plutôt que de les omettre complètement du livre.

##### Vue Web
* Tout document peut maintenant être ouvert dans la vue Web, pas seulement EPUB, HTML et Markdown.

##### Lisibilité
* Les en-têtes sont maintenant dessinés à une taille qui correspond à leur niveau, et les images et tableaux sont séparés du texte qui les entoure.

##### pb
* `pb --list-formats` répertorie tous les formats que pb peut lire.
* pb indique maintenant quel fichier il n'a pas pu lire et pourquoi.

#### Corrigé

##### Général
* Un livre réouvert au démarrage se lit maintenant immédiatement, au lieu de rester silencieux jusqu'à ce qu'il soit fermé et réouvert.
* Un document dont le fichier a disparu peut maintenant être supprimé de Tous les documents, au lieu de rester dans la liste quoi qu'il arrive.
* Correction d'un plantage lors de la fermeture de Paperback.
* La fermeture de Paperback masque maintenant la fenêtre immédiatement, au lieu de la laisser à l'écran pendant qu'elle s'enregistre.
* Les gros livres avec peu de mise en forme s'ouvrent maintenant environ deux fois plus vite.
* Les messages choisis dans un menu, comme « Ce document n'a pas d'audio », ne sont plus coupés par le lecteur d'écran avant que vous les entendiez.
* L'ouverture d'un document ne laisse plus Réouvrir le dernier fermé activé quand il n'y a rien à réouvrir.
* Paperback ne réessaie plus les documents de votre liste récente qui ont disparu, et limite le nombre de documents récents qu'il stocke.
* L'ancien fichier de paramètres INI est maintenant supprimé une fois qu'il a été converti au nouveau format.
* Les titres des dialogues de police et couleur, ainsi que le menu Exporter sous en vietnamien, sont maintenant traduits.
* La mise à jour amène maintenant la fenêtre relancée au premier plan, au lieu de la laisser derrière toutes les autres fenêtres dans `Alt+Tab`.
* L'enveloppe de texte s'applique maintenant immédiatement sur les gros documents, au lieu de recharger le tout.

##### Navigation
* `Alt+Left` revient maintenant à l'endroit d'où vous avez sauté, au lieu d'aller à une position plus ancienne.
* Les sons des signets ne se jouent maintenant que lorsque vous vous déplacez sur un signet, pas quand vous atterrissez sur la ligne sur laquelle il se trouve.
* La fermeture de la table des matières, de la liste des éléments et des dialogues Aller vous amène maintenant directement à la ligne sur laquelle vous atterrissez, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre.
* Aller à la ligne, Aller à la page et Aller au pourcentage refusent maintenant les nombres en dehors du document au lieu d'aller silencieusement ailleurs.
* NVDA ne coupe plus l'annonce quand un document n'a pas de pages.
* Appuyer sur OK dans la table des matières sans se déplacer va maintenant à l'entrée déjà sélectionnée.
* La table des matières, la liste des éléments et la liste des signets ne ralentissent plus et ne figent plus sur les livres avec des milliers d'entrées.
* Les flèches Haut et Bas mémorisent maintenant leur colonne par document, au lieu de la conserver quand vous changez d'onglet.

##### Livres audio
* La lecture audio utilise maintenant `Control+Space` sur macOS, car `Command+Space` appartient à Spotlight.

##### Documents PDF
* Correction des PDF exportés depuis Apple Pages qui se lisaient en tant que texte brut, sans les titres et les listes avec lesquels ils ont été écrits.
* Correction de la division des paragraphes et titres PDF à chaque ligne, et de la division des mots aux espaces.
* Correction des titres PDF numérotés qui s'exécutaient ensemble dans un seul titre.
* Correction des PDF dont l'arborescence de structure ne mène à aucun texte qui s'ouvrait vides.
* Les lignes définies dans une police à espacement fixe, comme le code, ne sont plus jointes en paragraphes.
* Les en-têtes et pieds de page ne sont plus lus sur chaque page des PDF non balisés.
* Les PDF qui balisent leurs en-têtes et pieds de page en tant que texte ordinaire ne répètent plus le titre et le numéro de page entre deux paragraphes sur chaque page.
* Les PDF affichent maintenant leur titre réel, au lieu du nom de leur fichier.

##### Livres MOBI/AZW3
* Les gros livres MOBI ne manquent plus de mémoire et ne sont plus tronqués après 20 Mo.
* Les livres MOBI et AZW3 s'ouvrent maintenant beaucoup plus vite.
* Correction de la perte de la liste des chapitres dans les livres MOBI.
* Correction du texte brouillé où les livres MOBI passent d'un enregistrement à l'autre.

##### Affichage web
* L'affichage web ne charge plus l'intégralité d'un énorme livre à la fois.
* L'affichage web affiche maintenant les documents en entier quand le lecteur les affiche en entier, au lieu de ne montrer qu'une tranche.

##### Autres formats
* Les livres FictionBook (.fb2) écrits en windows-1251, qui en est la majorité, s'ouvrent maintenant au lieu d'échouer complètement à la lecture.
* Les livres FictionBook qui utilisent un espace de noms ou une entité HTML qu'ils n'ont jamais déclarés s'ouvrent maintenant, au lieu d'être refusés comme cassés.
* Les livres en encodages anciens s'ouvrent maintenant beaucoup plus vite.
* Correction de l'ouverture de certains fichiers texte chinois en tant que texte brouillé.
* Les fichiers OpenDocument protégés par mot de passe demandent maintenant leur mot de passe, au lieu d'être signalés comme cassés.
* Les fichiers PowerPoint hérités protégés par mot de passe s'ouvrent maintenant, et les diapositives PowerPoint héritées ne perdent plus leur texte.
* Les fichiers texte brut enregistrés avec une extension `.rtf` s'ouvrent maintenant en tant que texte, au lieu d'échouer avec une erreur.
* Les mots de contrôle RTF n'apparaissent plus en tant que texte.

#### iOS et Android

Les applications iOS et Android ouvrent tous les formats que la version de bureau utilise, et incluent :

* Lecture à voix haute, avec votre choix de voix, de débit et de hauteur, un contrôle de débit de parole directement sur la barre de lecture, et une pause optionnelle entre les paragraphes.
* Lecture des livres audio DAISY, M4B et MP3, qui continue en arrière-plan et depuis l'écran de verrouillage.
* Navigation par titres, pages, liens, tableaux, listes et plus encore à partir de la barre de lecture, ainsi que la table des matières et Rechercher.
* Un minuteur de sommeil, un compteur de mots et une exportation de documents, plus un dictionnaire de parole sur iOS. Sur iOS, l'exportation se fait via la feuille de partage, de sorte qu'un livre peut aller vers une autre application ou vers Fichiers, dans un autre format ou exactement tel qu'il est.
* Options de taille de texte, d'espacement et de texte à contraste élevé.
* Des raccourcis clavier qui correspondent à la version de bureau.

### Version 0.9.2
* Les livres audio n'obligent plus votre lecteur d'écran à lire une suite d'espaces quand vous focalisez le champ de texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous les parcourez par section.
* Les livres audio indiquent maintenant leur durée réelle, au lieu de prétendre que chaque fichier qu'ils contiennent dure 24 heures.
* La fermeture de la Web View avec Échap ne déclenche plus une alerte de débogage après que vous ayez suivi un lien à l'intérieur.
* La copie après Sélectionner tout vous donne maintenant le document entier, au lieu de seulement la partie actuellement chargée.
* La recherche va maintenant directement à la ligne trouvée, au lieu de vous forcer à écouter le lecteur d'écran relire la fenêtre au fur et à mesure que la focus revient au livre.
* Correction des EPUB qui contiennent un bloc ZIP64 égaré refusant de s'ouvrir avec le message « Invalid local file header ».
* Correction des longs documents qui revenaient à leur début tandis qu'un lecteur d'écran les lisait en continu.
* Les liens dans la WebView vous mènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « File not found ».
* L'annonce automatique « Document reloaded » ne coupe plus votre lecteur d'écran au milieu d'une phrase, attendant plutôt qu'il finisse ce qu'il était en train de dire.
* L'onglet Général de la boîte de dialogue Paramètres traverse maintenant ses options dans l'ordre dans lequel elles apparaissent à l'écran, le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera maintenant toujours « Paperback » dans le menu Ouvrir avec, au lieu de la ligne d'accroche complète du programme.
* Le Nombre de mots et les Informations sur le document indiquent maintenant combien de fichiers contient un livre audio et quelle est sa durée totale.

### Version 0.9.1
* Les sons de signet et de note se jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, tirets longs et caractères similaires qui disparaissaient des documents RTF, fusionnant les mots environnants.
* Correction des images RTF qui fuyaient leurs données brutes dans le document sous forme de texte déformé.
* Correction du sous-menu Documents récents qui conservait les entrées obsolètes jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, les menus russes ont de nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus vite.
* Les documents ouverts sont maintenant enregistrés avec Windows, afin qu'ils apparaissent dans la liste de sauts de la barre des tâches et la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, pour correspondre aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback se souvient maintenant de la position, la taille et l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, de sorte que les messages qui comptent des choses se lisent correctement dans les langues qui en ont besoin de plus d'une.
* La sélection du ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de seulement son texte.
* Les noms d'actions de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document apparaît maintenant en premier dans la barre de titre, pour que les livres ouverts puissent être distingués dans la barre des tâches et Alt+Tab.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format supporté par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir le code source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte des documents est maintenant paginé, ce qui signifie que vous pouvez charger des livres contenant des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler tout comportement étrange.

##### Support des plates-formes
* Support ARM64 de Windows !
* Support natif de macOS !
* Un bouton pour passer en plein écran.

##### Dialogue Tous les documents
* Un bouton de localisation pour localiser les livres manquants qui ont juste changé de chemin.
* Un filtre d'état et une barre d'état, vous permettant de filtrer par état du document et de voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé depuis général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu retour à la ligne automatique et le raccourci clavier correspondant.
* Un bouton pour déterminer comment vous souhaitez afficher les tableaux, et harmonisation de l'affichage des tableaux dans tous les documents.

##### Navigation
* Support de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode de lecture des lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils sont conservés. Utilisez la barre oblique pour en créer un et la barre oblique inverse pour y accéder.

##### Nombre de mots
* Temps de lecture estimé dans la boîte de dialogue du nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique vraiment utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue du nombre de mots, le nombre de mots sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser tous les raccourcis clavier de l'application via une simple boîte de dialogue.
* Un raccourci clavier configurable pour restaurer Paperback à partir de la barre d'état système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Expansion de l'élément de menu d'export pour permettre l'export en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton d'annulation à la boîte de dialogue de mise à jour en cours.
* Le programme de mise à jour valide maintenant que le fichier téléchargé n'a pas été falsifié.

##### Affichage Web
* L'affichage Web s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Livres audio
* La possibilité de lire des livres audio, prenant actuellement en charge à la fois l'audio DAISY (y compris audio DAISY + texte) et les fichiers zip de fichiers audio.
* Raccourcis clavier et éléments de menu pour lire/mettre en pause la narration, avancer et reculer, et ajuster la quantité de recherche.
* Options pour synchroniser le curseur de lecture à la lecture audio, définir la quantité de recherche audio, et choisir si la recherche passant la fin d'un chapitre continue au suivant.

##### Documents CHM
* Support des listes, éléments de liste, figures et images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrigé

##### Général
* Les documents encodés dans les encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu d'afficher un ensemble de caractères corrompus.
* "Rouvrir le dernier fermé" tentant de rouvrir le fichier readme fourni.
* Votre onglet sélectionné ne s'obtenant pas correctement le focus après le redémarrage de Paperback.
* Gestion par Paperback des fichiers sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier place maintenant correctement le focus sur le fichier du stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration du document ; à la place, vous serez invité à confirmer quand l'un d'eux sera trouvé.
* Ouvrir le dossier contenant place maintenant correctement le focus sur le fichier donné dans l'explorateur.
* L'ouverture du fichier readme respecte maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback se redimensionne maintenant correctement sur les écrans haute résolution.
* Le menu se met à jour maintenant correctement, et le focus se déplace vers le contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de communication interprocessus sur Windows.
* Le titre du document actif sera maintenant lu lors du passage entre les onglets.
* Réduction de l'utilisation de la mémoire sur les documents volumineux en réduisant de moitié la taille des tables d'index par caractère interne.

##### Dialogue Tous les documents
* Échap ne ferme pas les dialogues Informations sur le document et Tous les documents.
* La barre de titre ne se met pas à jour après la fermeture d'un document depuis le dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lors de l'ouverture via `Shift+F1`.
* La suppression de documents du dialogue des fichiers récents ferme maintenant également leur onglet actif.
* Votre filtre de recherche est maintenant préservé après la suppression d'un document.

##### Navigation
* La navigation dans les pages annonçant un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les documents volumineux.
* Rechercher et Suivant ne respectant pas la fenêtre du document chargé dans les documents volumineux.

##### Signets
* Les sons de signet/note doivent maintenant se jouer exclusivement lorsque vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous menant au début de votre document.

##### Web View
* Le dialogue webview n'étant pas redimensionnable et s'affichant à une taille initiale très petite.
* Les images doivent maintenant s'afficher correctement dans la webview intégrée.

##### Mise à jour
* Le programme de mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre d'état.
* Chargement de livres DAISY avec des déclarations d'encodage erronées.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-latin.
* Les groupes RTF `\pict` afin que les données d'image intégrées ne s'échappent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des caractères indésirables dans le texte du livre.
* Liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de styles spécifiques aux paramètres régionaux ne rendaient pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF faussement étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne planteront plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support des pages aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement, Word hérité, Word moderne et Powerpoint moderne sont pris en charge, avec Powerpoint hérité prévu pour l'avenir.
* Ajout du support des documents Microsoft Word hérités !
* Ajout du support des présentations Powerpoint hérités !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF étiquetés !
* Ajout du raccourci `ctrl+q` pour quitter l'application.
* Ajout du support des livres compressés de Bookshare (DAISY et Word) !
* Le texte alternatif des images intégrées doit maintenant s'afficher correctement.
* Les documents CHM prennent maintenant correctement en charge la navigation des liens internes.
* Correction de l'option aller à la page étant décalée de 1.
* Correction de la touche d'échappement ne fonctionnant pas pour fermer le dialogue ouvrir sous.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applications.
* Correction du mauvais document recevant parfois le focus lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous alertent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec `g`/`shift+g` et `f`/`shift+f`, respectivement.
* Paperback respecte maintenant votre paramètre de mode sombre de l'application.
* Suppression du support DAISY XML, car il n'est plus nécessaire.
* Retour à la navigation par première lettre native Win32 dans l'arborescence de la table des matières.
* Le dialogue d'erreur de chargement affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvrira maintenant beaucoup plus rapidement et plus facilement.

### Version 0.8.2
* Ajout du support des pages aux documents RTF !
* Correction d'un bogue où l'ouverture de la webview dans les epub contenant des liens externes les activerait automatiquement.
* Correction d'un bogue où l'analyseur RTF ne mettrait pas d'espace entre les mots dans les cas rares.
* Correction des paragraphes étant divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF prennent maintenant en charge la navigation de base des liens et des titres !
* Les tabulations et sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour analyser les PDF, rendant le rendu PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour rouvrir le dernier document fermé.
* Le dialogue Tous les documents prend maintenant en charge la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bogues avec l'analyseur RTF.
* Correction des chemins de fichier contenant des caractères non-ASCII (tels que le bosniaque š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre, et espacement incorrect autour des mots en majuscules.
* Correction du chargement lent de documents lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les dialogues de confirmation.

### Version 0.8.0
* Ajout des traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'un programme de mise à jour automatique qui remplacera désormais votre version installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout du retour sonore facultatif lors de l'atteinte d'un signet ou d'une note, merci à Andre Louis pour les sons !
* Ajout de la prise en charge des documents RTF !
* Ajout de la prise en charge des documents DAISY XML.
* Ajout de la prise en charge des fichiers Open Document Text plats !
* Ajout de la prise en charge des présentations Open Document plats !
* Ajout de la prise en charge des séparateurs avec s et shift+s.
* Tout déplacement de plus de 300 caractères ajoutera désormais automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis la barre d'état système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans l'affichage Web.
* Correction du rendu incorrect des tableaux dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent désormais de leur existence lorsque vous tentez d'en charger un.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Division de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, offrant plus de fiabilité, de vitesse et moins de DLL.
* Réécriture de l'ensemble de l'application en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura désormais des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout de la prise en charge des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Shift+T, et appuyez sur Entrée pour en afficher un dans un affichage Web.
* Ajout d'une fonctionnalité de rendu Web de base ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le Web, utile pour le contenu tel que le formatage complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Effacer tout à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche désormais les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis la barre d'état système.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la table des matières dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des tables des matières d'Epub vous jetant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces dans les balises XML, HTML et pre.
* Correction d'une erreur de décalage d'une unité dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs de fin sur leurs lignes.
* Correction de divers problèmes d'analyse.
* Les éléments du menu liés aux signets ainsi que la liste des éléments sont désormais correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans différents formats de document.
* Amélioration du flux de traduction pour les contributeurs.
* De nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout de la prise en charge des PDF protégés par mot de passe !
* Ajout d'une fonctionnalité très basique pour aller à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et qu'il déplace votre curseur, cette position sera désormais mémorisée et pourra être accessible avec alt+left/right arrows.
* Ajout d'une liste d'éléments ! Actuellement, elle affiche uniquement une arborescence de tous les titres de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des tables des matières Epub contenant des chemins relatifs.
* Correction de certains documents Epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres Epub ne s'affichant pas correctement dans la boîte de dialogue de la table des matières.
* Correction de l'impossibilité d'utiliser la barre d'espace pour activer les boutons OK/Annuler dans la boîte de dialogue de la table des matières.
* Amélioration de la gestion des titres dans les documents Word.
* Vous recevrez désormais un retour parlé si la liste des documents récents est vide lorsque vous tentez d'ouvrir la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu de navigation dans une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, activée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels soit cyclique.
* Ajout d'une option au menu outils pour ouvrir le dossier contenant le document actuellement actif.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction basique de minuteur de veille, accessible avec Ctrl+Shift+S.
* Ajout de la prise en charge de l'analyse des livres numériques FB2 !
* Ajout de la prise en charge de l'analyse des présentations OpenDocument !
* Ajout de la prise en charge de l'analyse des fichiers OpenDocument Text !
* Les signets peuvent désormais marquer une ligne entière, ou marquer uniquement du texte spécifié. Si vous n'avez pas de sélection active lors de la création d'un signet, le comportement est comme avant la version 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent désormais avoir des notes de texte facultatives attachées ! Naviguez entre les signets contenant des notes avec N et Shift+N, ou ouvrez la boîte de dialogue des signets avec tous les signets, uniquement les notes, ou uniquement les signets sans notes sélectionnés avec des touches de raccourci spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus de préfixe ennuyeux "signet x".
* Les livres EPUB contenant du contenu HTML se faisant passer pour du XML seront désormais traités correctement.
* Correction du chargement de grands documents Markdown.
* Correction de la pression sur la barre d'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne reprenant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte de la boîte de dialogue d'accès au pourcentage qui ne mettait pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le code HTML à l'intérieur des blocs de code Markdown sera désormais rendu correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande pendant qu'une instance existante de Paperback s'exécute, vous n'obtiendrez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera désormais correctement chargée et enregistrée.
* Il est désormais possible de supprimer un signet directement depuis la boîte de dialogue des signets.
* Il est désormais possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension .paperback. Si un tel fichier se trouve dans le même répertoire qu'un fichier lors du chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement à l'aide d'un élément du menu outils.
* Les liens dans les documents sont maintenant entièrement pris en charge ! Utilisez k et shift+k pour vous déplacer vers l'avant et vers l'arrière dans les liens, et appuyez sur Entrée pour ouvrir/activer un lien.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le fichier binaire plus petit.
* Le contenu Markdown est désormais prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge ! Utilisez L et Shift+L pour parcourir les listes elles-mêmes, et I et Shift+I pour parcourir les éléments de liste.
* La touche Suppr du pavé numérique fonctionne désormais pour supprimer les documents de la barre d'onglets en plus de la touche Suppr normale.
* Paperback peut désormais éventuellement se minimiser dans votre barre système ! Cette option est désactivée par défaut, mais l'activation de cette option fera que l'option de minimisation du menu système placera Paperback dans votre barre système, où il pourra être restauré en cliquant sur l'icône créée.
* Paperback est désormais entièrement traduisible ! La liste des langues qu'il prend en charge est actuellement assez réduite, mais elle ne cesse de croître !
* Paperback dispose désormais d'un site Web officiel, à [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent désormais une table des matières basique, contenant tous les diapositives.
* Le chemin d'accès complet au document ouvert s'affiche désormais dans la boîte de dialogue d'informations du document.
* Le programme d'installation inclut désormais une option pour afficher le fichier Lisezmoi dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement développée ! Au lieu de vous afficher simplement les 10 derniers documents que vous avez ouverts, elle affichera désormais un nombre personnalisable, le reste des documents que vous avez jamais ouverts étant accessible par une petite boîte de dialogue.
* Diverses petites améliorations aux analyseurs dans l'ensemble, notamment en ajoutant une ligne vierge entre les diapositives dans les présentations PPTX, en corrigeant la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et en ajoutant des puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Correction de certains éléments de menu qui n'étaient pas désactivés quand aucun document n'était ouvert.
* Correction de l'orientation du curseur de pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichiers codés en URL et/ou des ID de fragment.
* Correction du suppression des espaces dans les en-têtes XHTML de façon étrange.
* Correction de la gestion des espaces à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Quand vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des en-têtes de votre document, et il vous la montrera dans la boîte de dialogue `ctrl+t`.
* Les documents HTML auront maintenant le titre défini dans la balise title, s'il existe. Sinon, ils continueront à utiliser le nom du fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région en direct pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est plus expédiée avec le programme, et plus de lecteurs d'écran seront désormais pris en charge, comme Microsoft Narrator.
* Passage des bibliothèques zip pour permettre l'ouverture d'une plus large gamme de livres epub.
* La boîte de dialogue vous demandant si vous voulez ouvrir votre document en tant que texte brut a été complètement refait, et elle vous permet maintenant d'ouvrir votre document en tant que texte brut, HTML ou Markdown.
* La boîte de dialogue aller au pourcentage inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour sauter à.
* L'analyseur HTML reconnaît maintenant dd, dt et dl comme des éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable unicode est maintenant considéré lors de la suppression des lignes blanches.
* Vous ne serez plus jamais demandé comment vous voulez ouvrir un fichier non reconnu à chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône optionnelle du menu Démarrer à l'installateur.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez naviguer vers l'avant et vers l'arrière avec `b` et `shift+b`, en définir un avec `control+shift+b`, et afficher une boîte de dialogue pour accéder à un signet spécifique avec `control+b`.
* Ajout d'un installateur à côté du fichier zip portable ! L'installateur installera Paperback dans votre répertoire Program Files, et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec les BOM devraient maintenant être décodés correctement, et le BOM ne sera plus affiché au début du texte.
* Ajout de beaucoup plus d'informations à la barre d'état. Elle vous montrera maintenant votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne seront plus affichés dans la sortie texte.
* Si vous passez un chemin relatif à Paperback sur la ligne de commande, il le résoudra maintenant correctement.
* Le mouvement en pourcentage est maintenant géré par sa propre boîte de dialogue basée sur un curseur, accessible avec `control+shift+g`.
* Les documents sans titres ou auteurs connus auront maintenant toujours une valeur par défaut.
* La logique de sauvegarde de la position est maintenant beaucoup plus intelligente et ne devrait écrire sur le disque que lorsque c'est absolument nécessaire.
* Le document sur lequel vous aviez le focus quand vous avez fermé Paperback est maintenant mémorisé après le redémarrage de l'application.
* L'entrée dans les boîtes de dialogue aller à la ligne et aller à la page devrait maintenant être assainie plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation des en-têtes dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de l'utilisation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments TOC imbriqués dans les livres Epub plaçant votre curseur à la mauvaise position.
* Correction d'un plantage à la fermeture de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne !
* Il est maintenant possible de faire un don au développement de Paperback, soit via le nouvel élément donate dans le menu aide, soit via le lien sponsor this project en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant être capable de charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Passage des bibliothèques PDF à celle utilisée dans Chromium, ce qui permet une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. Exécuter `paperback.exe` avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur `Delete` sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue aller à la page.
* Autoriser la tabulation du contenu du document à votre liste de documents ouverts.
* Correction des raccourcis clavier de titre ouvrant parfois des documents récents si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union souples inutiles de la sortie texte.
* Correction de la navigation des en-têtes vous mettant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis clavier pour naviguer par en-têtes dans le contenu HTML, y compris les livres epub et les documents markdown. Ces raccourcis ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epubs avec des noms de fichiers encodés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré.
* Un message est maintenant énoncé si le document ne supporte pas une table des matières ou des sections, au lieu que les éléments du menu soient désactivés.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et la prise en charge des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés entre les redémarrages de l'application. Cela est configurable via le nouvel élément d'options du menu Outils.
* Ajout de `Shift+F1` pour ouvrir le fichier readme directement dans Paperback lui-même.

### Version 0.1.0
* Première version.
