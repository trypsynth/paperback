<!-- machine-translated from doc/readme.md (source-hash: 2afffa4b3f966e85; sections: 84030068,db723a70,df2f4c18,14335443,d44bf4c8,6c87c514,94527a25,ca4819ea,a9eba369,e9860ee8,3b8321f8); please review and edit as needed -->

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

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous constaterez peut-être que les longs paragraphes sont tronqués lors du défilement vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également affectée. Il s'agit d'un bogue dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et c'est un bogue qui a pris un certain temps avant qu'une correction soit apportée, compte tenu de l'enthousiasme de Vispero pour répondre aux problèmes des logiciels open source.

La solution de contournement, finalement trouvée par le groupe de discussion JAWS après des mois d'attente, consiste à modifier `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif au lieu d'avancer. Avec ces deux paramètres en place, le défilement devrait fonctionner correctement.

### JAWS et les messages de Paperback

Paperback dit des choses comme « No pages. » ou « This document has no audio. » comme notifications d'accessibilité, ce qui permet à un lecteur d'écran de les énoncer par-dessus ce qu'il dit. JAWS n'agit sur celles-ci que lorsque « Enable accessible notification events » est activé pour l'application, et sur certaines machines, ce n'est pas le cas.

Si JAWS ne dit rien quand vous appuyez sur une touche qui devrait rapporter quelque chose, ouvrez Settings Center avec Paperback en avant (`Insert+6`), recherchez « notification », et cochez « Enable accessible notification events ». Cela écrit le paramètre dans `paperback.jcf`, de sorte qu'il s'applique à Paperback seul.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Archives de bandes dessinées (`.cbz`)
* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Livres électroniques FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Pages de manuel, à la fois `man` et BSD `mdoc` (`.1` à `.9`, `.man`, `.roff`, et les formes compressées de chacun)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres audio M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Livres audio MP3 (`.mp3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Documents Windows Write (`.wri`)
* Fichiers WinHelp (`.hlp`)
* Fichiers texte brut et journaux (`.txt`, `.log`)

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

1.0 est la première version sur les cinq plates-formes : Windows, macOS, Linux, iOS et Android, avec les applications iOS et Android dans l'App Store et Google Play.

#### Ajouté

##### Général
* Support de Linux, sous forme d'AppImage ou de tar.gz, avec intégration de bureau pour que les documents s'ouvrent depuis votre gestionnaire de fichiers.
* Marquez le début d'une sélection avec `Alt+F9`, copiez tout de là jusqu'où vous êtes arrivé avec `Alt+F10`, et revenez à la marque avec `Alt+Shift+F9`, pour copier une longue portion de texte sans la parcourir avec Maj. Ces trois options sont sous Outils > Sélectionner et copier.
* Le raccourci `=` annonce maintenant la page ainsi que le pourcentage, par exemple « 15 %, page 30 », et reste comme avant pour les documents sans numéros de page.
* La boîte À propos affiche maintenant la licence de Paperback et tous les traducteurs.
* Une traduction en ukrainien.

##### Nouveaux formats
* Archives de bandes dessinées (`.cbz`).
* Audiolivres M4B, divisés en chapitres.
* Pages de manuel, à la fois `man` et BSD `mdoc`, compressées ou non.
* Audiolivres MP3, divisés en chapitres lorsque le fichier en contient.
* Fichiers Windows Write (`.wri`).
* Fichiers WinHelp (`.hlp`).
* Documents Word 6 et Word 95.

##### ROC
* Les pages PDF numérisées peuvent maintenant être reconnues avec la ROC intégrée à Windows et macOS. Appuyez sur `Enter` sur une page numérisée pour la reconnaître, ou utilisez la ROC par lot (`Ctrl+Shift+O`) pour une plage de pages.

##### Navigation
* Les formules MathML dans EPUB et HTML sont rendues sous forme d'AsciiMath en utilisant MathCAT. Utilisez `M` ou `Shift+M` pour naviguer dans les formules, puis `Enter` ou `Space` pour ouvrir le MathML original dans la vue Formule.
* Un bouton Trouver tout dans la boîte de dialogue Rechercher, listant chaque ligne avec une correspondance pour que vous puissiez accéder directement à celle que vous voulez.
* Les vues Tableaux, Listes et Pages dans la liste des éléments (`F7`).
* Aller à la ligne, Aller à la page et Aller au pourcentage acceptent maintenant `+n` et `-n` pour se déplacer relativement à votre position actuelle.
* Les livres EPUB, MOBI et CHM sans titres propres obtiennent maintenant la navigation par titres à partir de leur table des matières.
* Les livres KF8 (AZW3) supportent maintenant la navigation par section.
* Les pages EPUB qui ne sont qu'une image affichent maintenant une ligne pour cela, afin que vous puissiez vous y positionner au lieu de les sauter.

##### Audiolivres
* Contrôles de vitesse de lecture, de demi-vitesse à trois fois plus rapide. Utilisez `Ctrl+Shift+.` et `Ctrl+Shift+,`, ou le menu Outils.
* Les signets et les notes dans les livres audio seuls se souviennent maintenant du moment exact où vous les avez placés.
* Position suivante et précédente (`Alt+Left` et `Alt+Right`) fonctionnent maintenant dans les audiolivres.
* La progression dans un audiolivre est maintenant mesurée par son enregistrement, donc Aller au pourcentage et la barre d'état correspondent à la distance réelle parcourue.

##### Documents récents
* Un élément Effacer les documents récents dans le sous-menu Documents récents.

##### Documents PDF
* Un paramètre pour garder chaque ligne d'un PDF séparée, au lieu de les joindre en paragraphes.
* Les images et les figures dans les PDF sont maintenant annoncées.
* Les PDF qui contiennent une structure de lecture mais ne balisent aucune de leurs images annoncent maintenant ces images, au lieu de les exclure entièrement du livre.

##### Vue Web
* N'importe quel document peut maintenant être ouvert dans la vue web, pas seulement EPUB, HTML et Markdown.

##### Lisibilité
* Les titres sont maintenant dessinés à une taille qui correspond à leur niveau, et les images et les tableaux sont séparés du texte qui les entoure.

##### pb
* `pb --list-formats` liste tous les formats que pb peut lire.
* pb indique maintenant quel fichier il n'a pas pu lire et pourquoi.

#### Corrigé

##### Général
* Correction d'un plantage lors de la fermeture de Paperback.
* La fermeture de Paperback masque maintenant la fenêtre immédiatement, au lieu de la laisser à l'écran pendant qu'elle s'enregistre.
* L'ouverture d'un document ne laisse plus la fonction Rouvrir le dernier fermé activée quand il n'y a rien à rouvrir.
* Paperback ne réessaie plus indéfiniment d'accéder aux documents de votre liste récente qui ont disparu, et limite le nombre de documents récents qu'il conserve.
* L'ancien fichier de paramètres INI est maintenant supprimé une fois qu'il a été converti au nouveau format.
* Les titres des boîtes de dialogue police et couleur, ainsi que le menu Exporter sous en vietnamien, sont maintenant traduits.
* La mise à jour amène désormais la fenêtre relancée au premier plan, au lieu de la laisser derrière toutes les autres fenêtres dans `Alt+Tab`.
* L'habillage du texte s'applique maintenant immédiatement sur les gros documents, au lieu de recharger tout le document.

##### Navigation
* `Alt+Left` revient maintenant à l'endroit d'où vous aviez sauté, plutôt qu'à une position plus ancienne.
* Les sons des signets ne se jouent maintenant que lorsque vous vous déplacez sur un signet, pas quand vous arrivez sur la ligne où il se trouve.
* La fermeture de la table des matières, de la liste des éléments et des boîtes de dialogue Aller à vous amène maintenant directement à la ligne sur laquelle vous arrivez, au lieu de vous obliger à écouter le lecteur d'écran relire la fenêtre.
* Aller à la ligne, Aller à la page et Aller au pourcentage refusent maintenant les nombres en dehors du document au lieu d'aller silencieusement ailleurs.
* NVDA ne coupe plus l'annonce quand un document n'a pas de pages.
* Appuyer sur OK dans la table des matières sans bouger va maintenant à l'entrée qui était déjà sélectionnée.
* La table des matières, la liste des éléments et la liste des signets ne ralentissent plus et ne figent plus sur les livres avec des milliers d'entrées.
* Les flèches Haut et Bas se souviennent maintenant de leur colonne par document, au lieu de la conserver quand vous changez d'onglet.

##### Livres audio
* La lecture audio utilise maintenant `Control+Space` sur macOS, puisque `Command+Space` appartient à Spotlight.

##### Documents PDF
* Correction des PDF exportés à partir d'Apple Pages qui s'affichaient en tant que texte brut, sans aucun des titres et listes avec lesquels ils ont été rédigés.
* Correction des paragraphes et titres PDF se divisant à chaque ligne, et des mots se divisant aux espaces.
* Correction des titres PDF numérotés se regroupant en un seul titre.
* Correction des PDF dont l'arborescence des structures ne mène à aucun texte s'ouvrant vide.
* Les en-têtes et pieds de page ne sont plus énoncés sur chaque page des PDF non balisés.
* Les PDF qui balisent leurs en-têtes et pieds de page comme du texte ordinaire ne répètent plus le titre et le numéro de page entre deux paragraphes sur chaque page.
* Les PDF affichent maintenant leur vrai titre, au lieu de leur nom de fichier.
* Les lignes définies dans une police à espacement fixe, comme le code, ne sont plus regroupées en paragraphes.

##### Livres MOBI/AZW3
* Les gros livres MOBI ne manquent plus de mémoire et ne sont plus tronqués après 20 Mo.
* Les livres MOBI et AZW3 s'ouvrent maintenant beaucoup plus rapidement.
* Correction des livres MOBI perdant leur liste de chapitres.
* Correction du texte brouillé quand les livres MOBI passent d'un enregistrement à l'autre.

##### Vue Web
* La vue Web ne charge plus l'intégralité d'un énorme livre d'un coup.
* La vue Web affiche maintenant les documents entiers quand le lecteur les affiche entiers, au lieu de n'en montrer qu'une portion.

##### Autres formats
* Les livres FictionBook (.fb2) écrits en windows-1251, ce qui est le cas pour la plupart, s'ouvrent maintenant au lieu de ne pas se lire du tout.
* Les livres FictionBook qui utilisent un espace de noms ou une entité HTML qu'ils n'ont jamais déclarés s'ouvrent maintenant, au lieu d'être refusés comme cassés.
* Les livres dans les anciens encodages s'ouvrent maintenant beaucoup plus rapidement.
* Correction de certains fichiers texte en chinois s'ouvrant en tant que texte brouillé.
* Les fichiers OpenDocument protégés par mot de passe demandent maintenant leur mot de passe, au lieu d'être signalés comme cassés.
* Les fichiers PowerPoint hérités protégés par mot de passe s'ouvrent maintenant, et les diapositives PowerPoint héritées ne perdent plus leur texte.
* Les fichiers texte brut enregistrés avec une extension `.rtf` s'ouvrent maintenant en tant que texte, au lieu d'échouer avec une erreur.
* Les mots-clés RTF ne s'affichent plus en tant que texte.

#### iOS et Android

Les applications iOS et Android ouvrent tous les formats que fait le bureau, et incluent :

* Lecture à haute voix, avec votre choix de voix, de débit et de hauteur, un contrôle de débit de parole directement sur la barre de lecture, et une pause facultative entre les paragraphes.
* Lecture des livres audio DAISY, M4B et MP3, qui continue en arrière-plan et depuis l'écran de verrouillage.
* Navigation par titres, pages, liens, tableaux, listes et plus à partir de la barre de lecture, plus la table des matières et Rechercher.
* Un minuteur de mise en veille, un comptage de mots et une export de documents, plus un dictionnaire de parole sur iOS.
* Options de taille de texte, d'espacement et de texte à contraste élevé.
* Les raccourcis clavier qui correspondent au bureau.

### Version 0.9.2
* Les livres audio ne font plus lire à votre lecteur d'écran une série d'espaces quand vous concentrez le champ de texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous les parcourez par section.
* Les livres audio signalent maintenant leur vraie longueur, au lieu de prétendre que chaque fichier qu'ils contiennent dure 24 heures.
* La fermeture de la vue Web avec Échap ne lève plus une alerte de débogage après avoir suivi un lien à l'intérieur.
* La copie après Sélectionner tout vous donne maintenant le document entier, au lieu de seulement la partie actuellement chargée.
* Rechercher va maintenant directement à la ligne qu'elle a trouvée, au lieu de vous obliger à écouter le lecteur d'écran relire la fenêtre à nouveau alors que le focus revient au livre.
* Correction des EPUB qui portent un bloc ZIP64 égaré refusant de s'ouvrir avec « En-tête de fichier local invalide ».
* Correction des longs documents revenant à leur début tandis qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la WebView vous amènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « Fichier non trouvé ».
* L'annonce automatique « Document rechargé » ne coupe plus votre lecteur d'écran au milieu d'une phrase, attendant plutôt qu'il finisse ce qu'il disait.
* L'onglet Général de la boîte de dialogue Paramètres parcourt maintenant ses options dans l'ordre dans lequel elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera maintenant toujours « Paperback » dans le menu Ouvrir avec, au lieu du slogan complet du programme.
* Le comptage des mots et les informations sur les documents affichent maintenant le nombre de fichiers qu'un livre audio contient et la durée totale qu'il occupe.

### Version 0.9.1
* Les sons des signets et des notes se jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, tirets cadratin et caractères similaires disparaissant des documents RTF, fusionnant les mots environnants au passage.
* Correction des images RTF qui fuyaient leurs données brutes dans le document sous forme de texte garbled.
* Correction du sous-menu Documents récents qui conservait les entrées obsolètes jusqu'à ce qu'autre chose le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, de sorte que les menus russes ont à nouveau un accès clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés avec Windows, de sorte qu'ils apparaissent dans la liste de saut de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, correspondant aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback se souvient maintenant de la position, de la taille et de l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, de sorte que les messages qui comptent les choses se lisent correctement dans les langues qui nécessitent plus d'une forme.
* La sélection du ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de seulement son texte.
* Les noms d'actions de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, de sorte que les livres ouverts peuvent être distingués dans la barre des tâches et `Alt+Tab`.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format supporté par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile pour éditer du Markdown par exemple.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée avec ceci.

##### Support de plateforme
* Support ARM64 Windows !
* Support natif macOS !
* Un bouton de basculement plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre de statut et une barre de statut, de sorte que vous pouvez filtrer par statut du document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé à partir de général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu retour à la ligne automatique et touche d'accès rapide suivante.
* Un bouton de basculement pour déterminer comment vous souhaitez afficher les tableaux, et unification de la façon dont les tableaux sont affichés dans tous les documents.

##### Navigation
* Support pour naviguer par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode parcourir dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inversée pour sauter vers celui-ci.

##### Comptage des mots
* Temps de lecture estimé dans la boîte de dialogue comptage des mots, ainsi que la capacité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue comptage des mots, le nombre de mots que vous avez sélectionnés sera maintenant affiché.

##### Raccourcis clavier
* La capacité de personnaliser chaque raccourci clavier de l'application via une simple boîte de dialogue.
* Un raccourci clavier configurable pour restaurer Paperback à partir du plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Expansion de l'élément de menu export pour permettre l'export en HTML et Markdown, en plus du texte brut.

##### Mises à jour
* Un bouton d'annulation à la boîte de dialogue mise à jour en cours.
* Le programme de mise à jour valide maintenant que le fichier téléchargé n'a pas été falsifié.

##### Vue Web
* La vue web s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Livres audio
* La capacité de lire des livres audio, supportant actuellement à la fois l'audio DAISY (y compris audio DAISY + texte) et les fichiers audio zippés.
* Raccourcis clavier et éléments de menu pour lire/mettre en pause la narration, chercher en avant et en arrière, et ajuster le montant de la recherche.
* Options pour synchroniser le curseur de lecture à la lecture audio, définir le montant de la recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support pour les listes, les éléments de liste, les figures et les images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrigé

##### Général
* Les documents encodés en CJK legacy, comme GBK, Big5 et Shift_JIS, s'affichent désormais correctement au lieu de s'afficher en caractères corrompus.
* « Rouvrir le dernier fermé » tentant de rouvrir le readme fourni.
* Votre onglet sélectionné ne recevant pas correctement le focus après le redémarrage de Paperback.
* Gestion des fichiers sur les lecteurs réseau Windows par Paperback : appuyer sur afficher le fichier dans le dossier place désormais correctement le focus sur le fichier du stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus chargés de force lors de la restauration du document ; au lieu de cela, vous serez invité à confirmer lorsqu'un est trouvé.
* Ouvrir le dossier contenant place désormais correctement le focus sur le fichier donné dans l'explorateur.
* L'ouverture du readme respecte désormais votre langue sélectionnée.
* L'interface utilisateur de Paperback s'ajuste désormais correctement sur les écrans haute résolution.
* Le menu se met désormais à jour correctement et le focus se déplace vers le contrôle de texte lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de communication interprocessus (IPC) sur Windows.
* Le titre du document actif sera désormais lu lors du changement d'onglet.
* Réduction de l'utilisation de la mémoire sur les documents volumineux en réduisant de moitié la taille des tables d'index par caractère interne.

##### Dialogue Tous les documents
* Échap ne fermant pas les dialogues Infos document et Tous les documents.
* La barre de titre ne se mettant pas à jour après fermeture d'un document à partir du dialogue de tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lorsqu'il est ouvert via `Shift+F1`.
* La suppression de documents du dialogue des récents fermera désormais aussi leur onglet actif.
* Votre filtre de recherche est désormais conservé après la suppression d'un document.

##### Navigation
* La navigation entre pages annonçant un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les documents volumineux.
* Rechercher et Rechercher suivant ne respectant pas la fenêtre du document chargé dans les documents volumineux.

##### Signets
* Les sons de signet/note doivent désormais se reproduire correctement uniquement lorsque vous naviguez sur un mot en contenant.

##### Lisibilité
* L'application du retour à la ligne vous envoyant au début de votre document.

##### Affichage Web
* Le dialogue de webview n'étant pas redimensionnable et s'ouvrant à une très petite taille initiale.
* Les images doivent désormais s'afficher correctement dans la webview intégrée.

##### Mise à jour
* La mise à jour affiche désormais correctement le contenu des balises de code markdown dans les notes de publication.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre d'état.
* Chargement de livres DAISY avec des déclarations d'encodage frauduleuses.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-Latin.
* Les groupes RTF `\pict` afin que les données d'image intégrées ne s'écoulent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi scindant les balises HTML et mettant du contenu indésirable dans le texte du livre.
* Liens dans les livres Mobi legacy.
* Amélioration majeure de l'analyse AZW3.

##### Documents Word
* Documents Word avec noms de style spécifiques aux paramètres régionaux ne rendant pas correctement leurs en-têtes.

##### Documents HTML/XHTML
* Éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback bascule désormais vers l'extraction en texte brut pour les PDF incorrectement étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne planteront plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout de la prise en charge des pages aux livres epub.
* Ajout de la prise en charge des documents Microsoft Office chiffrés. Actuellement, Word legacy, Word moderne et Powerpoint moderne sont pris en charge, avec Powerpoint legacy prévu pour l'avenir.
* Ajout de la prise en charge des documents Microsoft Word legacy !
* Ajout de la prise en charge des présentations Powerpoint legacy !
* Ajout de la prise en charge des livres mobi et AZW3 !
* Ajout de la prise en charge des fichiers PDF étiquetés !
* Ajout du raccourci `ctrl+q` pour quitter l'application.
* Ajout de la prise en charge des livres compressés de Bookshare (DAISY et Word) !
* Le texte alternatif des images intégrées doit désormais s'afficher correctement.
* Les documents CHM prennent désormais correctement en charge la navigation par lien interne.
* Correction de l'erreur de décalage de 1 pour aller à la page.
* Correction de la touche Échap ne fonctionnant pas pour fermer le dialogue d'ouverture.
* Correction du menu contextuel du lecteur ne s'affichant pas lors d'un clic droit ou de la touche Applications.
* Correction du mauvais document recevant parfois le focus lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous alertent de leur existence.
* Il est désormais possible de naviguer dans les images et les figures avec `g`/`shift+g` et `f`/`shift+f`, respectivement.
* Paperback respecte désormais votre paramètre de mode sombre d'application.
* Suppression de la prise en charge DAISY XML, car elle n'est plus nécessaire.
* Retour à la navigation Win32 native des premières lettres dans l'arbre de la table des matières.
* Le dialogue de chargement d'erreur affiche désormais des messages d'erreur plus détaillés.
* La webview s'ouvrira désormais beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Ajout de la prise en charge des pages aux documents RTF !
* Correction d'un bogue où l'ouverture de la webview dans les epubs contenant des liens externes les activait automatiquement.
* Correction d'un bogue où l'analyseur RTF n'insérait pas d'espace entre les mots dans de rares cas.
* Correction des paragraphes divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF bénéficient désormais d'une prise en charge basique de la navigation par lien et en-tête !
* Les onglets et sauts de ligne RTF sont désormais rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant le rendu PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour rouvrir le dernier document fermé.
* Le dialogue Tous les documents prend désormais en charge la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bogues avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (comme le š, č, ć, ž bosniaque) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre et de l'espacement incorrect autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les dialogues de confirmation.

### Version 0.8.0
* Ajout de traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'un programme de mise à jour automatique qui remplace désormais votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout de commentaires sonores optionnels pour atteindre un signet ou une note, merci à Andre Louis pour les sons !
* Ajout de la prise en charge des documents RTF !
* Ajout de la prise en charge des documents DAISY XML.
* Ajout de la prise en charge des fichiers Open Document Text plats !
* Ajout de la prise en charge des présentations Open Document plats !
* Ajout de la prise en charge des séparateurs avec s et shift+s.
* Tout mouvement supérieur à 300 caractères s'ajoutera désormais automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis la barre d'état système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans Web View.
* Correction du rendu incorrect des tableaux dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent désormais de leur existence lorsque vous tentez d'en charger un.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Division de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, conduisant à plus de fiabilité, de vitesse et à moins de DLL.
* Réécriture de l'ensemble de l'application en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura désormais des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout de la prise en charge des tableaux pour les documents HTML et XHTML !
Naviguez entre les tableaux à l'aide de T et shift+t, et appuyez sur Entrée pour en afficher un dans un rendu web.
* Ajout d'une fonction de rendu web basique ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un rendu basé sur le web, utile pour du contenu comme les formats complexes ou les exemples de code.
* Ajout d'une traduction russe, merci à Ruslan Gulmagomedov !
* Ajout d'un bouton Tout effacer à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche désormais les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis la barre d'état système.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse TOC dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue Rechercher qui ne se fermait pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction de la TOC d'Epub vous redirigeant occasionnellement vers le mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans XML, HTML et les balises pre.
* Correction d'une erreur d'un élément hors de portée dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs de fin sur leurs lignes.
* Correction de divers problèmes d'analyse.
* Les éléments de menu liés aux signets ainsi que la liste des éléments sont désormais correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de documents.
* Amélioration du flux de travail de traduction pour les contributeurs.
* De nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout de la prise en charge des PDF protégés par mot de passe !
* Ajout d'une fonction très basique pour aller à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et que cela déplace votre curseur, cette position sera désormais mémorisée et pourra être navigée avec les flèches alt+left/right.
* Ajout d'une liste d'éléments ! Actuellement, elle affiche uniquement un arbre de tous les titres de votre document ou une liste de liens, mais il est prévu de l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des TOC Epub contenant des chemins relatifs.
* Correction de certains documents Epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres Epub ne s'affichant pas correctement dans la boîte de dialogue TOC.
* Correction de l'impossibilité d'utiliser la barre d'espacement pour activer les boutons OK/annuler dans la boîte de dialogue TOC.
* Amélioration de la gestion des titres dans les documents Word.
* Vous recevrez désormais un retour parlé si la liste des documents récents est vide lorsque vous essayez d'ouvrir la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu de navigation sous une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, activée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels soit bouclante.
* Ajout d'une option au menu outils pour ouvrir le dossier contenant le document actuellement ciblé.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction de minuterie de sommeil basique, accessible avec Ctrl+Shift+S.
* Ajout de la prise en charge de l'analyse des livres électroniques FB2!
* Ajout de la prise en charge de l'analyse des présentations OpenDocument!
* Ajout de la prise en charge de l'analyse des fichiers OpenDocument Text!
* Les signets peuvent désormais marquer une ligne entière, ou marquer uniquement du texte spécifié. Si aucune sélection n'est active lors du placement d'un signet, le comportement est similaire à la version antérieure à 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent désormais avoir des notes textuelles optionnelles attachées! Naviguez entre les signets contenant des notes avec N et Shift+N, ou ouvrez la boîte de dialogue des signets avec tous les signets, uniquement les notes, ou uniquement les signets sans notes sélectionnés avec des touches de raccourci spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus de préfixe ennuyeux « signet x ».
* Les livres EPUB contenant du contenu HTML prétendant être du XML seront désormais gérés correctement.
* Correction du chargement de grands documents Markdown.
* Correction de l'activation du bouton OK en appuyant sur la barre d'espace dans l'arborescence de la table des matières.
* Correction de la gestion des espaces au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne reprenant pas parfois le focus lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue d'accès au pourcentage qui ne mettait pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le code HTML à l'intérieur des blocs de code Markdown sera désormais rendu correctement.
* Si le chargement d'un livre avec un paramètre de ligne de commande prend plus de 5 secondes alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus d'erreur.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera désormais chargée et enregistrée correctement.
* Il est maintenant possible de supprimer un signet directement à partir de la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré porte le même nom que le fichier avec une extension `.paperback`. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement à l'aide d'un élément du menu outils.
* Les liens à l'intérieur des documents sont maintenant entièrement pris en charge! Utilisez k et Shift+K pour vous déplacer d'avant en arrière, et appuyez sur Entrée pour ouvrir/activer un lien.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le fichier binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge! Utilisez L et Shift+L pour naviguer entre les listes elles-mêmes, et I et Shift+I pour parcourir les éléments de liste.
* Le bouton Supprimer du pavé numérique fonctionne désormais pour supprimer des documents de la barre d'onglets en plus du bouton Supprimer normal.
* Paperback peut désormais se minimiser optionnellement dans votre plateau système! Cette option est désactivée par défaut, mais l'activer fera en sorte que l'option de minimisation dans le menu système place Paperback dans votre plateau, pouvant être restauré en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traduisible! La liste des langues qu'il prend en charge est actuellement assez réduite, mais elle s'agrandit constamment!
* Paperback dispose désormais d'un site Web officiel, à [paperback.dev](https://paperback.dev)!
* Les documents PPTX affichent désormais une table des matières basique contenant toutes les diapositives.
* Le chemin d'accès complet au document ouvert s'affichera désormais dans la boîte de dialogue d'informations sur le document.
* Le programme d'installation inclut désormais une option pour afficher le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement développée! Au lieu de simplement afficher les 10 derniers documents que vous avez ouverts, elle affichera désormais un nombre personnalisable, les autres documents que vous avez jamais ouverts étant accessibles via une petite boîte de dialogue.
* Diverses petites améliorations aux analyseurs dans l'ensemble, notamment l'ajout d'une ligne vierge entre les diapositives dans les présentations PPTX, la correction de la gestion des nouvelles lignes à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Prise en charge des documents Microsoft Word ajoutée !
* Prise en charge des présentations PowerPoint ajoutée !
* Certains éléments de menu ne sont plus désactivés lorsqu'aucun document n'est ouvert.
* L'orientation du curseur de pourcentage de navigation a été corrigée.
* La table des matières dans les livres Epub avec des chemins de fichiers codés en URL et/ou des identifiants de fragment a été corrigée.
* Les espaces supprimés des en-têtes XHTML de manière étrange ont été corrigés.
* La gestion des espaces à l'intérieur des balises pre imbriquées dans les documents HTML a été corrigée.
* Les documents HTML et Markdown prennent désormais en charge la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des en-têtes de votre document, et l'affichera dans la boîte de dialogue `ctrl+t`.
* Les documents HTML auront désormais le titre défini dans la balise title, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région active pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est plus livrée avec le programme, et plus de lecteurs d'écran seront désormais pris en charge, comme Microsoft Narrator.
* Passage à des bibliothèques zip pour permettre l'ouverture d'une plus large gamme de livres epub.
* La boîte de dialogue vous demandant si vous souhaitez ouvrir votre document en tant que texte brut a été complètement refaite, et elle vous permet désormais d'ouvrir votre document en tant que texte brut, HTML ou Markdown.
* La boîte de dialogue de navigation à un pourcentage inclut désormais un champ de texte vous permettant d'entrer manuellement un pourcentage auquel accéder.
* L'analyseur HTML reconnaîtra désormais dd, dt et dl comme éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable Unicode est désormais pris en compte lors de la suppression des lignes vides.
* Vous ne serez plus demandé comment vous souhaitez ouvrir un fichier non reconnu à chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Une icône de menu Démarrer optionnelle a été ajoutée au programme d'installation.
* La table des matières devrait être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez désormais que l'élément parent.
* La table des matières dans certains documents CHM a été corrigée.
* La table des matières dans les livres Epub 3 avec des chemins absolus a été corrigée.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Prise en charge des fichiers CHM ajoutée !
* Prise en charge des signets ajoutée ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez naviguer vers l'avant et vers l'arrière avec `b` et `shift+b`, en définir un avec `control+shift+b`, et afficher une boîte de dialogue pour accéder à un signet spécifique avec `control+b`.
* Un programme d'installation a été ajouté aux côtés du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec des BOM devraient désormais être décodés correctement, et le BOM ne s'affichera plus au début du texte.
* Beaucoup plus d'informations ont été ajoutées à la barre d'état. Elle affichera désormais votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne seront plus affichés dans la sortie de texte.
* Si vous passez un chemin relatif à Paperback sur la ligne de commande, il sera désormais résolu correctement.
* Le mouvement en pourcentage est désormais géré par sa propre boîte de dialogue basée sur un curseur, accessible avec `control+shift+g`.
* Les documents sans titres ou auteurs connus auront désormais toujours une valeur par défaut.
* La logique de sauvegarde de position est désormais beaucoup plus intelligente et ne devrait écrire sur le disque que lorsque c'est absolument nécessaire.
* Le document sur lequel vous aviez le focus lorsque vous avez fermé Paperback est désormais mémorisé entre les redémarrages de l'application.
* L'entrée dans les boîtes de dialogue d'accès à la ligne et d'accès à la page devrait désormais être purgée de manière plus stricte.
* La navigation dans la table des matières des livres epub 3 avec des chemins relatifs dans leurs manifestes a été corrigée.

### Version 0.3.0
* La table des matières dans les livres epub avec des manifestes codés en URL a été corrigée.
* La navigation des en-têtes dans les documents HTML contenant des caractères Unicode multibytes a été corrigée.
* L'utilisation élevée du processeur dans les documents avec de longs titres due à une régression dans wxWidgets a été corrigée.
* Le chargement des fichiers texte UTF-8 a été corrigé.
* Les éléments de table des matières imbriqués dans les livres Epub plaçant votre curseur à la mauvaise position ont été corrigés.
* Un plantage à la fermeture de l'application dans certains cas a été corrigé.
* Une case à cocher a été ajoutée à la boîte de dialogue d'options pour activer ou désactiver le retour à la ligne automatique !
* Il est désormais possible de faire un don au développement de Paperback, soit via le nouvel élément donate du menu d'aide, soit via le lien sponsor this project en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront désormais toujours un titre, et Paperback devrait désormais être capable de charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront désormais toujours un titre, même si les métadonnées sont manquantes.
* Passage à des bibliothèques PDF utilisées dans Chromium, conduisant à une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez désormais avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez désormais appuyer sur delete sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Le nombre total de pages a été ajouté à l'étiquette de page dans la boîte de dialogue d'accès à la page.
* Permet de passer d'un onglet du contenu du document à votre liste de documents ouverts.
* Les raccourcis clavier d'en-tête ouvrant parfois des documents récents si vous en aviez assez ont été corrigés.
* Paperback supprimera désormais les tirets conditionnels inutiles de la sortie de texte.
* La navigation des en-têtes vous plaçant parfois au mauvais caractère a été corrigée.

### Version 0.2.0
* Ajout de la prise en charge des documents markdown !
* Ajout de la prise en charge des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis clavier pour naviguer par titres dans le contenu HTML, y compris les livres epub et les documents markdown. Ces raccourcis ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epub avec des noms de fichiers codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec du XHTML intégré dedans.
* Un message est maintenant prononcé si le document ne prend pas en charge une table des matières ou des sections, au lieu que les éléments de menu soient désactivés.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et la prise en charge des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés lors des redémarrages de l'application. Ceci est configurable via le nouvel élément options du menu Outils.
* Ajout de `Shift+F1` pour ouvrir le fichier readme directement dans Paperback.

### Version 0.1.0
* Version initiale.
