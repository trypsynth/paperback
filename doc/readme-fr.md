<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc); please review and edit as needed -->

# Paperback - version 0.9.2

## Introduction

Paperback est un lecteur d'ebooks et de documents léger, rapide et accessible pour tous, des lecteurs occasionnels aux utilisateurs avancés. Il est conçu pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans surcharge.

## Configuration système requise

Paperback s'exécute actuellement sur Windows 10/11 et toutes les versions modernes d'ARM macOS. Les applications iOS et Android natives sont en développement actif, avec des versions de test publiques prévues peu après la sortie 0.9.0 du bureau, avant une sortie unifiée 1.0 couvrant les quatre plates-formes.

## Fonctionnalités

* Complètement autonome, ne nécessitant aucun logiciel à installer sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface simple avec onglets, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans chaque document que vous ouvrez.
* Peut éventuellement mémoriser les documents que vous aviez ouverts lors de la fermeture du programme et les restaurer au lancement suivant.
* Inclut des fonctionnalités de navigation similaires à celles que l'on trouve dans le mode de navigation web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut une boîte de dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et la prise en charge des expressions régulières.
* Peut s'exécuter entièrement de manière portable ou être installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un très large éventail de formats de fichiers courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les lecteurs d'écran majeurs. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous constaterez peut-être que les longs paragraphes sont tronqués lors d'un défilement avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe courant est également affectée. Il s'agit d'un bogue dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et un problème qui a pris pas mal de temps à surmonter étant donné l'enthousiasme de Vispero à répondre aux problèmes de logiciels open source.

La solution de contournement, finalement découverte via le groupe de discussion JAWS après des mois d'attente, est d'éditer `paperback.jcf` et de définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif plutôt que d'avancer. Avec les deux paramètres en place, le défilement devrait fonctionner correctement.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiobooks M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Fichiers texte brut et fichiers journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation basée sur le clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que `Ctrl+G`, `Ctrl+W`, et `Alt+Left`/`Right` sont déjà revendiqués par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document courant.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des Documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, c'est dans le menu de l'application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (lorsque supporté par le document courant).
* `=` : Annoncer votre pourcentage de lecture courant.
* `Alt+Left` (macOS : `Cmd+[`) : Revenir en arrière dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l'historique de navigation.
* `[` : Section précédente.
* `]` : Section suivante.
* `Shift+H` : En-tête précédent.
* `H` : En-tête suivant.
* `Shift+1` à `Shift+6` : En-tête précédent de niveau 1-6.
* `1` à `6` : En-tête suivant de niveau 1-6.
* `Shift+P` : Page précédente.
* `P` : Page suivante.
* `Shift+B` : Marque-page précédent.
* `B` : Marque-page suivant.
* `/` : Définir votre marque-page temporaire.
* `\` : Aller à votre marque-page temporaire.
* `Shift+N` : Note précédente.
* `N` : Note suivante.
* `Ctrl+B` : Aller à tous les marque-pages et notes.
* `Ctrl+Alt+B` : Aller aux marque-pages uniquement.
* `Ctrl+Alt+M` : Aller aux notes uniquement.
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le texte de la note à la position courante.
* `Shift+K` : Lien précédent.
* `K` : Lien suivant.
* `Shift+G` : Image précédente.
* `G` : Image suivante.
* `Shift+F` : Figure précédente.
* `F` : Figure suivante.
* `Shift+T` : Tableau précédent.
* `T` : Tableau suivant.
* `Shift+S` : Séparateur précédent.
* `S` : Séparateur suivant.
* `Shift+L` : Liste précédente.
* `L` : Liste suivante.
* `Shift+I` : Élément de liste précédent.
* `I` : Élément de liste suivant.
* `Shift+,` : Aller au début du conteneur courant (liste ou tableau).
* `,` : Aller au-delà de la fin du conteneur courant (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document courant.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu courant dans la Vue Web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document courant en texte brut.
* `Ctrl+Shift+B` : Basculer le marque-page à la sélection/curseur courant.
* `Ctrl+Shift+N` : Ajouter ou modifier la note du marque-page à la sélection/curseur courant.
* `Ctrl+Alt+W` : Basculer le retour à la ligne.
* `Ctrl+Space` : Lire/pausser la narration audio.
* `'` : Avancer la narration audio.
* `;` : Reculer la narration audio.
* `Ctrl+'` : Augmenter la durée d'avance audio.
* `Ctrl+;` : Diminuer la durée d'avance audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, dans le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de veille.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de donation dans votre navigateur par défaut.

### Touches supplémentaires de la vue du document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet du document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsque sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, et d'autres sont ajoutées régulièrement. Une liste complète suit ci-dessous.

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
* Vietnamien

## Crédits
### Développement
* Quin Gillespie : développeur principal et fondateur du projet.
* Aryan Choudhary : contributeur principal.

### Donations
Les personnes suivantes ont fait des donations de diverses tailles au développement de Paperback. Si vous faites une donation, votre nom ne sera pas automatiquement ajouté ici ; je n'ajoute que les personnes qui souhaitent que leur donation soit publique.

Note : Je considère un parrainage GitHub public comme une raison d'inclusion automatique dans cette liste.

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

### Version 0.9.2
* Les livres audio ne font plus lire à votre lecteur d'écran une série d'espaces quand vous mettez le focus sur le champ de texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous les parcourez par section.
* Les livres audio signalent maintenant leur durée réelle, au lieu de prétendre que chaque fichier dure 24 heures.
* Fermer la Web View avec Échap ne génère plus une alerte de débogage après que vous ayez suivi un lien à l'intérieur.
* Copier après Sélectionner tout vous donne maintenant le document entier, au lieu de seulement la partie actuellement chargée.
* Chercher va maintenant directement à la ligne trouvée, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre alors que le focus revient au livre.
* Correction des EPUB qui contiennent un bloc ZIP64 égaré et refusaient de s'ouvrir avec le message "Invalid local file header".
* Correction des documents longs qui revenaient au début pendant qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la WebView vous mènent maintenant à la section à laquelle ils pointent, au lieu d'échouer avec "File not found".
* L'annonce automatique "Document rechargé" n'interrompt plus votre lecteur d'écran au milieu d'une phrase, en attendant plutôt qu'il finisse ce qu'il disait.
* L'onglet Général de la boîte de dialogue Paramètres navigue maintenant à travers ses options dans l'ordre où elles apparaissent à l'écran, le canal de mise à jour se trouvant directement après l'option de vérification des mises à jour.
* Windows affiche maintenant toujours "Paperback" dans le menu Ouvrir avec, au lieu du slogan complet du programme.
* Le Nombre de mots et Infos sur le document affichent maintenant combien de fichiers contient un livre audio et sa durée totale.

### Version 0.9.1
* Les sons de signet et de note jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, tirets cadratins et caractères similaires qui disparaissaient des documents RTF, fusionnant les mots environnants.
* Correction des images RTF qui divulguaient leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents qui conservait les entrées obsolètes jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, donc les menus du russe ont à nouveau accès au clavier.
* Les gros documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés avec Windows, ils apparaissent donc dans la liste de saut de la barre des tâches et la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, en accord avec les applications mobiles et, sur macOS, la convention de la plateforme.
* Paperback se souvient maintenant de sa position de fenêtre, sa taille et son état maximisé entre les exécutions.
* Les formes plurielles sont maintenant traduites, donc les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plus d'une forme.
* Sélectionner le fichier ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de seulement son texte.
* Les noms d'action de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, les livres ouverts peuvent donc être distingués dans la barre des tâches et Alt+Tab.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format supporté par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile pour éditer du Markdown par exemple.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler les bizarreries trouvées avec ceci.

##### Support de plateforme
* Support de Windows ARM64 !
* Support natif de macOS !
* Un bouton bascule en plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton de localisation pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre d'état et une barre d'état, vous permettant de filtrer par statut du document et de voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet de lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé de général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne automatique et une touche de raccourci correspondante.
* Un bouton pour déterminer comment vous voulez afficher les tableaux, et unifier la façon dont les tableaux sont affichés dans tous les documents.

##### Navigation
* Support pour la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode de navigation des lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez barre oblique pour en définir un et barre oblique inversée pour sauter à celui-ci.

##### Nombre de mots
* Temps de lecture estimé dans la boîte de dialogue du nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique vraiment utile.
* Si une sélection est active quand vous ouvrez la boîte de dialogue du nombre de mots, le nombre de mots sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser tous les raccourcis clavier de l'application via une boîte de dialogue simple.
* Un raccourci clavier configurable pour restaurer Paperback depuis le plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Exportation
* Expansion de l'élément de menu d'exportation pour permettre l'exportation vers HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton d'annulation pour la boîte de dialogue de mise à jour en cours.
* La mise à jour valide maintenant que le fichier téléchargé n'a pas été altéré.

##### Web View
* La webview s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support pour les livres DAISY 2.0.
* Support pour la lecture audio DAISY 2.02.

##### Livres audio
* La possibilité de lire des livres audio, supportant actuellement DAISY audio (y compris DAISY audio + texte) et les archives de fichiers audio.
* Raccourcis clavier et éléments de menu pour lire/pause la narration, rechercher en avant et en arrière, et ajuster le montant de la recherche.
* Options pour synchroniser le curseur de lecture avec la lecture audio, définir le montant de la recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support pour les listes, les éléments de liste, les figures et les images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Correction

##### Général
* Les documents codés dans les encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu d'un fatras de mojibake.
* "Rouvrir le dernier fermé" tentant de rouvrir le readme fourni.
* Votre onglet sélectionné ne s'obtenant pas correctement le focus après le redémarrage de Paperback.
* La gestion des fichiers de Paperback sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier met maintenant correctement le focus sur le fichier du stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus chargés de force lors de la restauration du document ; au lieu de cela, vous serez invité à confirmer quand on en trouve un.
* Ouvrir le dossier contenant met maintenant le focus sur le fichier donné dans l'explorateur.
* L'ouverture du readme respectera maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adapte maintenant correctement sur les écrans haute DPI.
* Le menu s'actualise maintenant correctement et le focus se déplace vers le contrôle de texte lors de l'ouverture de l'aide dans Paperback.
* Basculé vers une méthode beaucoup plus sécurisée d'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du basculement entre les onglets.
* Utilisation réduite de la mémoire sur les gros documents en réduisant de moitié la taille des tables d'index internes par caractère.

##### Boîte de dialogue Tous les documents
* Échap ne fermant pas les boîtes de dialogue Infos sur le document et Tous les documents.
* La barre de titre ne s'actualisant pas après la fermeture d'un document à partir de la boîte de dialogue de tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lors de son ouverture via Shift+F1.
* Supprimer les documents de la boîte de dialogue des récents fermera maintenant aussi leur onglet actif.
* Votre filtre de recherche est maintenant préservé après la suppression d'un document.

##### Navigation
* La navigation de page annonçant le texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les gros documents.
* Chercher et Chercher suivant ne respectant pas la fenêtre du document chargé dans les gros documents.

##### Signets
* Les sons de signet/note devraient maintenant jouer correctement exclusivement quand vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous propulsant au début de votre document.

##### Web View
* La boîte de dialogue webview ne pouvait pas être redimensionnée et s'affichait à une taille initiale très petite.
* Les images devraient maintenant s'afficher correctement dans la webview intégrée.

##### Mise à jour
* La mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des infos incorrectes dans la barre d'état.
* Chargement des livres DAISY avec des déclarations d'encodage fausses.

##### Documents RTF
* Analyse des documents RTF avec des caractères non-Latin.
* Les groupes RTF `\pict` afin que les données d'image intégrées ne fuient plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi scindant les balises HTML et mettant des ordures dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Amélioration majeure de l'analyse AZW3.

##### Documents Word
* Les documents Word avec des noms de style spécifiques à la langue ne rendant pas correctement leurs en-têtes.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF falsement balisés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne feront plus planter Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support des pages aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement les anciens Word, les nouveaux Word et les nouveaux PowerPoint sont supportés, avec les anciens PowerPoint prévus pour l'avenir.
* Ajout du support des documents Microsoft Word hérités !
* Ajout du support des présentations PowerPoint hérités !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF balisés !
* Ajout du raccourci ctrl+q pour quitter l'application.
* Ajout du support des livres compressés de Bookshare (DAISY et Word) !
* Le texte alternatif des images intégrées devrait maintenant s'afficher correctement.
* Les documents CHM supportent maintenant correctement la navigation par lien interne.
* Correction du fait que aller à la page soit décalé de 1.
* Correction de la touche Échap ne fonctionnant pas pour fermer la boîte de dialogue d'ouverture en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou sur la touche Applications.
* Correction du mauvais document étant parfois en focus lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous avertissent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec g/shift+g et f/shift+f, respectivement.
* Paperback respecte maintenant votre paramètre de mode sombre de l'application.
* Suppression du support DAISY XML, car il n'est plus nécessaire.
* Retour à la navigation native Win32 par première lettre dans l'arborescence de la table des matières.
* La boîte de dialogue d'erreur de chargement affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvre maintenant beaucoup plus rapidement et sans heurts.

### Version 0.8.2
* Ajout du support des pages aux documents RTF !
* Correction d'un bug où l'ouverture de la webview dans les epubs contenant des liens externes les activerait automatiquement.
* Correction d'un bug où l'analyseur RTF n'ajouterait pas un espace entre les mots dans de rares cas.
* Correction des paragraphes étant scindés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF disposent maintenant du support de base de la navigation par lien et en-tête !
* Les tabulations et sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant l'analyse PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de Ctrl+Shift+T pour rouvrir le dernier document fermé.
* La boîte de dialogue Tous les documents supporte maintenant la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bugs de l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (comme le bosniaque š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre, et espacement incorrect autour des mots capitalisés.
* Correction du chargement lent des documents lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Ajout des traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'une mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout des commentaires sonores optionnels pour atteindre un signet ou une note, merci Andre Louis pour les sons !
* Ajout du support des documents RTF !
* Ajout du support des documents DAISY XML.
* Ajout du support des fichiers Flat Open Document Text !
* Ajout du support des présentations Flat Open Document !
* Ajout du support des séparateurs avec s et shift+s.
* Tout mouvement supérieur à 300 caractères s'ajoute automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis le plateau système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans la Web View.
* Correction des tableaux ne s'affichant pas correctement dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent maintenant de leur existence quand vous tentez d'en charger un.
* Incorporation correcte des informations de version dans l'exécutable Paperback.
* Division de la boîte de dialogue des options en onglets pour la facilité d'utilisation et la navigation.
* Basculement vers Hayro pour l'analyse des PDF, conduisant à plus de fiabilité, de vitesse et de moins de DLL.
* Réécriture de l'application entière en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques comme couper et coller.

### Version 0.7.0
* Ajout du support des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Shift+T, et appuyez sur Entrée pour en afficher un dans une webview.
* Ajout d'une fonction de rendu web basique ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un rendu basé sur le web, utile pour le contenu avec un formatage complexe ou des exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Effacer tout à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version quand une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis le plateau système.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la table des matières dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des tables des matières de epub vous jetant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans les balises XML, HTML et pré.
* Correction d'une erreur hors d'un dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs à la fin de leurs lignes.
* Correction de divers problèmes d'analyse.
* Les éléments de menu relatifs aux signets ainsi que la liste des éléments sont maintenant correctement désactivés quand aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de documents.
* Amélioration du flux de travail de traduction pour les contributeurs.
* Beaucoup de refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout du support des PDF protégés par mot de passe !
* Ajout d'une fonction très basique aller à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et qu'il déplace votre curseur, cette position sera maintenant mémorisée et peut être navigué avec alt+left/right arrows.
* Ajout d'une liste d'éléments ! Actuellement, elle affiche uniquement une arborescence de tous les en-têtes de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des tables des matières Epub contenant des chemins relatifs.
* Correction de certains documents epub ne montrant pas un titre ou un auteur.
* Correction de l'absence des titres de certains chapitres epub dans la boîte de dialogue de la table des matières.
* Correction du fait que vous ne puissiez pas utiliser la barre d'espace pour activer les boutons OK/annuler dans la boîte de dialogue de la table des matières.
* Amélioration de la gestion des en-têtes dans les documents Word.
* Vous obtiendrez maintenant une rétroaction parlée si la liste des documents récents est vide quand vous essayez de faire apparaître la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu d'aller dans une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, coché par défaut.
* Ajout d'une option pour faire envelopper la navigation par éléments structurels.
* Ajout d'une option au menu des outils pour ouvrir le dossier contenant du document actuellement en focus.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction de minuteur de sommeil basique, accessible avec Ctrl+Shift+S.
* Ajout du support pour l'analyse des ebooks FB2 !
* Ajout du support pour l'analyse des présentations OpenDocument !
* Ajout du support pour l'analyse des fichiers OpenDocument Text !
* Les signets peuvent maintenant marquer une ligne entière ou seulement un texte spécifié. Si vous n'avez pas de sélection active lors du placement d'un signet, le comportement est comme avant la 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées ! Naviguez entre les signets contenant des notes avec N et Shift+N, ou affichez la boîte de dialogue des signets avec tous les signets, seulement les notes ou seulement les non-notes sélectionnés avec des touches spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus de préfixe ennuyeux "signet x".
* Les livres Epub contenant du contenu HTML prétendant être du XML seront maintenant traités correctement.
* Correction du chargement des gros documents Markdown.
* Correction de l'appui sur la barre d'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces blancs au début des balises pré dans les documents HTML et XHTML.
* Correction du contrôle de texte ne regagnant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue d'aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si le chargement d'un livre avec un paramètre de ligne de commande prend plus de 5 secondes alors qu'une instance de Paperback existe déjà, vous n'obtiendrez plus d'erreur.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant correctement chargée et enregistrée.
* Il est maintenant possible de supprimer un signet directement depuis la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension .paperback. Si un tel fichier se trouve dans le même répertoire qu'un fichier lors du chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement à l'aide d'un élément du menu des outils.
* Les liens à l'intérieur des documents sont maintenant complètement supportés ! Utilisez k et shift+k pour avancer et reculer à travers eux, et appuyez sur Entrée pour ouvrir/activer un.
* Beaucoup de refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité afin d'être conforme à CommonMark avant rendu.
* La navigation par listes et leurs éléments est maintenant complètement supportée ! Utilisez L et Shift+L pour parcourir les listes elles-mêmes, et I et Shift+I pour parcourir les éléments de liste.
* La suppression du pavé numérique fonctionne maintenant pour supprimer les documents de la barre des onglets en plus de la suppression normale.
* Paperback peut maintenant en option se minimiser dans votre plateau système ! Cette option est désactivée par défaut, mais l'activation fera que l'option minimiser du menu système mette Paperback dans votre plateau, pouvant être restauré en cliquant sur l'icône générée.
* Paperback est maintenant complètement traduisible ! La liste des langues qu'il supporte est actuellement assez petite, mais elle ne cesse de croître !
* Paperback a maintenant un site officiel, sur [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert s'affichera maintenant dans la boîte de dialogue infos sur le document.
* Le programme d'installation inclut maintenant une option pour afficher le readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement étendue ! Au lieu de vous montrer simplement les 10 derniers documents que vous avez ouverts, elle affichera maintenant un nombre personnalisable, les documents restants que vous avez jamais ouverts étant accessibles via une petite boîte de dialogue.
* Diverses petites améliorations apportées aux analyseurs sur toute la planche, notamment en plaçant une ligne vierge entre les diapositives dans les présentations PPTX, en corrigeant la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents word, et en ajoutant des puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Correction de certains éléments de menu ne se désactivant pas avec aucun document ouvert.
* Correction de l'orientation du curseur d'aller au pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichier codés en URL et/ou des ID de fragment.
* Correction des espaces blancs étant supprimés des en-têtes XHTML de façons bizarres.
* Correction de la gestion des espaces blancs à l'intérieur des balises pré imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Quand vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des en-têtes de votre document, et l'affichera dans la boîte de dialogue ctrl+t.
* Les documents HTML auront maintenant le titre tel que défini dans la balise titre, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région dynamique pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est expédiée aux côtés du programme, et plus de lecteurs d'écran seront maintenant supportés, tels que Microsoft Narrator.
* Passage à une bibliothèque zip différente pour permettre l'ouverture d'une plus large gamme de livres epub.
* La boîte de dialogue vous demandant si vous voulez ouvrir votre document en tant que texte brut a été complètement refaite, et permet maintenant d'ouvrir votre document en tant que texte brut, HTML ou Markdown.
* La boîte de dialogue d'aller au pourcentage inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage auquel sauter.
* L'analyseur HTML reconnaîtra maintenant dd, dt et dl en tant qu'éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable Unicode est maintenant considéré lors de la suppression des lignes vierges.
* On ne vous demandera plus comment vous voulez ouvrir un fichier non reconnu chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône du menu Démarrer optionnelle au programme d'installation.
* La table des matières devrait maintenant être plus nette dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez sauter en avant et en arrière à travers eux avec b et shift+b, en définir un avec control+shift+b, et faire apparaître une boîte de dialogue pour sauter à un signet spécifique avec control+b.
* Ajout d'un programme d'installation aux côtés du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec BOM devraient maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout de bien plus d'informations à la barre d'état. Elle affichera maintenant votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises de script et de style, ne s'afficheront plus dans la sortie texte.
* Si vous passez un chemin relatif à Paperback sur la ligne de commande, il le résoudra maintenant correctement.
* Le mouvement en pourcentage est maintenant géré par sa propre boîte de dialogue basée sur un curseur, accessible avec control+shift+g.
* Les documents sans titres ou auteurs connus auront maintenant une valeur par défaut.
* La logique d'économie de position est maintenant beaucoup plus intelligente et ne devrait écrire sur le disque que lorsque c'est absolument nécessaire.
* Le document auquel vous aviez le focus quand vous avez fermé Paperback est maintenant mémorisé entre les redémarrages de l'application.
* L'entrée dans les boîtes de dialogue d'aller à la ligne et d'aller à la page devrait maintenant être désinfectée plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation par en-tête dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de l'utilisation élevée du processeur dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments de table des matières imbriquées dans les livres Epub plaçant votre curseur à la mauvaise position.
* Correction d'une panne à la fermeture de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est maintenant possible de faire un don au développement de Paperback, soit via le nouvel élément de don dans le menu d'aide, soit via le lien sponsor ce projet au bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant pouvoir charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Passage aux bibliothèques PDF utilisées dans Chromium, conduisant à une analyse PDF beaucoup plus fiable.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier alors qu'il s'exécute déjà ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur supprimer sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue d'aller à la page.
* Permettre la tabulation du contenu du document à votre liste de documents ouverts.
* Correction des raccourcis de titre ouvrant parfois les documents récents si vous en aviez assez.
* Paperback supprimera maintenant les tirets doux inutiles de la sortie texte.
* Correction de la navigation par en-tête vous mettant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis pour naviguer par en-têtes dans le contenu HTML, y compris les livres epub et les documents markdown. Ces raccourcis ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epubs avec des noms de fichier codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec du XHTML intégré à l'intérieur.
* Un message est maintenant parlé si le document ne supporte pas une table des matières ou des sections, au lieu que les éléments de menu soient désactivés.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Chercher, la rendant beaucoup plus facile à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés entre les redémarrages de l'application. C'est configurable via le nouvel élément des options dans le menu des outils.
* Ajout de shift+f1 pour ouvrir le readme directement dans Paperback lui-même.

### Version 0.1.0
* Sortie initiale.
