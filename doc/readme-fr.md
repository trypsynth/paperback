<!-- machine-translated from doc/readme.md (source-hash: d49e7044d9856698); please review and edit as needed -->

# Paperback - version 0.9.1

## Introduction

Paperback est un lecteur léger, rapide et accessible pour les ebooks et documents, conçu pour tous, des lecteurs occasionnels aux utilisateurs expérimentés. Il est conçu pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans surcharge.

## Configuration requise

Paperback fonctionne actuellement sur Windows 10/11 et toutes les versions modernes d'ARM macOS. Des applications natives iOS et Android sont en cours de développement actif, avec des versions de test publiques prévues peu après la version desktop 0.9.0, avant une version unifiée 1.0 couvrant les quatre plateformes.

## Fonctionnalités

* Complètement autonome, ne nécessitant l'installation d'aucun logiciel sur votre ordinateur pour commencer la lecture.
* Incroyablement rapide, même sur du matériel ancien.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte pour chaque document que vous ouvrez.
* Mémorise optionnellement les documents que vous aviez ouverts lors de la fermeture du programme et les restaure au lancement suivant.
* Inclut une fonctionnalité de navigation similaire à celle trouvée en mode de navigation web dans de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut un dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et le support des expressions régulières.
* Peut être exécuté entièrement de manière portable ou installé avec les associations de fichiers configurées automatiquement.
* Supporte un ensemble massif de formats de fichier courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il y a cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs Braille

Si vous utilisez JAWS avec un afficheur Braille, vous pouvez constater que les longs paragraphes sont tronqués lors du défilement avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe courant est également affectée. C'est un bogue dans la gestion par JAWS du contrôle de texte RICHEDIT50W, non quelque chose dans Paperback lui-même, et un bogue dont la correction a pris longtemps à émerger compte tenu de l'enthousiasme de Vispero pour répondre aux problèmes des logiciels open source.

La solution, finalement découverte par le groupe de discussion JAWS après des mois d'attente, est de modifier `paperback.jcf` et de définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif plutôt que d'avancer. Avec ces deux paramètres en place, le défilement devrait fonctionner correctement.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Fichiers texte brut et journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation axée sur le clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que Ctrl+G, Ctrl+W, et Alt+Left/Right sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, cela se trouve dans le menu de l'application).

### Menu Aller à

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher le suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher le précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (si supporté par le document actuel).
* `=` : Annoncer votre pourcentage de lecture actuel.
* `Alt+Left` (macOS : `Cmd+[`) : Revenir dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l'historique de navigation.
* `[` : Section précédente.
* `]` : Section suivante.
* `Shift+H` : En-tête précédent.
* `H` : En-tête suivant.
* `Shift+1` à `Shift+6` : En-tête précédent de niveau 1-6.
* `1` à `6` : En-tête suivant de niveau 1-6.
* `Shift+P` : Page précédente.
* `P` : Page suivante.
* `Shift+B` : Signet précédent.
* `B` : Signet suivant.
* `/` : Définir votre signet temporaire.
* `\` : Aller à votre signet temporaire.
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
* `Shift+S` : Séparateur précédent.
* `S` : Séparateur suivant.
* `Shift+L` : Liste précédente.
* `L` : Liste suivante.
* `Shift+I` : Élément de liste précédent.
* `I` : Élément de liste suivant.
* `Shift+,` : Aller au début du conteneur actuel (liste ou tableau).
* `,` : Aller au-delà de la fin du conteneur actuel (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document actuel.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu actuel dans la vue Web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document actuel en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/curseur actuel.
* `Ctrl+Shift+N` : Ajouter ou modifier la note du signet à la sélection/curseur actuel.
* `Ctrl+Alt+W` : Basculer le retour à la ligne.
* `Ctrl+Space` : Lire/pause la narration audio.
* `'` : Avancer la narration audio.
* `;` : Reculer la narration audio.
* `Ctrl+'` : Augmenter la durée de recherche audio.
* `Ctrl+;` : Diminuer la durée de recherche audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, sous le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de veille.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de dons dans votre navigateur par défaut.

### Touches supplémentaires de la vue de document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet de document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsque vous êtes sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, d'autres étant ajoutées en permanence. Une liste complète suit ci-dessous.

Pour savoir comment contribuer, veuillez consulter notre [Guide de traduction](translating.md).

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

## Remerciements
### Développement
* Quin Gillespie : développeur principal et fondateur du projet.
* Aryan Choudhary : contributeur principal.

### Dons
Les personnes suivantes ont fait des dons de diverses tailles au développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici, j'ajoute uniquement les personnes qui souhaitent que leur donation soit rendue publique.

Remarque : Je considère qu'un parrainage GitHub public justifie une inclusion automatique dans cette liste.

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

## Changelog

### Version 0.9.1
* Les sons des signets et des notes sont maintenant lus sur macOS.
* Les livres DAISY lisent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, tirets longs et caractères similaires disparaissant des documents RTF, fusionnant les mots environnants au fur et à mesure.
* Correction des images RTF divulguant leurs données brutes dans le document sous forme de texte garbled.
* Correction du sous-menu Documents récents conservant les anciennes entrées jusqu'à ce que quelque chose d'autre le reconstruit.
* Les accélérateurs clavier sont de retour dans chaque traduction, les menus russes ont à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, donc ils apparaissent dans la liste de sauts de la barre des tâches et la liste récente du menu Démarrage.
* Options a été renommé en Paramètres, correspondant aux applications mobiles et, sur macOS, la convention de la plateforme.
* Paperback se souvient maintenant de la position, la taille et l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plusieurs formes.
* La sélection du ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de simplement son texte.
* Les noms d'actions de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, les livres ouverts peuvent être distingués dans la barre des tâches et `Alt+Tab`.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format supporté par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile pour éditer du Markdown par exemple.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée avec cela.

##### Support de la plateforme
* Support ARM64 Windows!
* Support natif macOS!
* Un basculement du mode plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton de localisation pour localiser les livres manquants qui vient de changer de chemin.
* Un filtre de statut et une barre de statut, vous permettant de filtrer par statut de document et de voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet de lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé de la section générale) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne automatique et un raccourci clavier ultérieur.
* Un basculement pour déterminer comment vous voulez afficher les tableaux, et unifié l'affichage des tableaux dans les documents.

##### Navigation
* Support pour naviguer par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode de navigation dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour y accéder.

##### Compte de mots
* Temps de lecture estimé dans la boîte de dialogue de décompte des mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique vraiment utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue de décompte des mots, le nombre de mots que vous avez sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La capacité de personnaliser chaque raccourci clavier dans l'application par le biais d'une simple boîte de dialogue.
* Un raccourci clavier configurable pour restaurer Paperback à partir du plateau système.

##### Langues
* Néerlandais, finlandais et polonais.

##### Exporter
* Expansion de l'élément du menu d'exportation pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Updater
* Un bouton d'annulation à la boîte de dialogue de mise à jour en cours.
* Le programme de mise à jour valide maintenant que le fichier téléchargé n'a pas été falsifié.

##### Web View
* La vue web s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support pour les livres DAISY 2.0.
* Support pour la lecture audio DAISY 2.02.

##### Livres audio
* La capacité de lire des livres audio, supportant actuellement à la fois l'audio DAISY (y compris l'audio DAISY + texte) et les zips de fichiers audio.
* Les raccourcis clavier et les éléments de menu pour lire/mettre en pause la narration, chercher en avant et en arrière, et ajuster la quantité de recherche.
* Des options pour synchroniser le curseur de lecture à la lecture audio, définir la quantité de recherche audio et choisir si chercher au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support pour les listes, les éléments de liste, les figures et les images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrigé

##### Général
* Les documents codés dans les anciens encodages CJK, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu d'un tas de mojibake.
* « Réouvrir la dernière fermeture » tentant de réouvrir le fichier readme fourni.
* L'onglet sélectionné ne reçoit pas correctement le focus après le redémarrage de Paperback.
* La gestion des fichiers Paperback sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier focus maintenant correctement le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus chargés de force lors de la restauration des documents ; à la place, vous serez invité à confirmer quand on en trouve un.
* Ouvrir le dossier contenant focus maintenant le fichier donné dans l'explorateur.
* L'ouverture du fichier readme respectera maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adapte maintenant correctement sur les écrans haute DPI.
* Le menu se met à jour maintenant correctement, et le focus passe au contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de l'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du passage d'un onglet à l'autre.
* Consommation mémoire réduite sur les grands documents en réduisant de moitié la taille des tableaux d'index internes par caractère.

##### Boîte de dialogue Tous les documents
* Échapper ne ferme pas les boîtes de dialogue Document Info et Tous les documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document à partir de la boîte de dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lors de l'ouverture via `Shift+F1`.
* La suppression de documents de la boîte de dialogue des récentes fermera maintenant aussi leurs onglets actifs.
* Votre filtre de recherche est maintenant conservé après la suppression d'un document.

##### Navigation
* La navigation par page annonçant un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les gros documents.
* Trouver et Trouver suivant ne respectant pas la fenêtre du document chargé dans les gros documents.

##### Signets
* Les sons de signet/note devraient maintenant se lire correctement exclusivement lorsque vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous envoie au début de votre document.

##### Web View
* La boîte de dialogue webview n'étant pas redimensionnable et apparaissant à une très petite taille initiale.
* Les images devraient maintenant s'afficher correctement dans le webview intégré.

##### Updater
* Le programme de mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre de statut.
* Chargement des livres DAISY avec des déclarations d'encodage fausses.

##### Documents RTF
* Analyse des documents RTF avec des caractères non-latins.
* Les groupes RTF `\pict` pour que les données d'image intégrées ne s'échappent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des ordures dans le texte du livre.
* Les liens dans les anciens livres Mobi.
* Analyse AZW3 largement améliorée.

##### Documents Word
* Les documents Word avec des noms de style spécifiques à la locale ne rendant pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback se rabat maintenant sur l'extraction de texte brut pour les PDF faussement marqués.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne plantent plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support des pages aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement, le Word hérité, le Word moderne et PowerPoint moderne sont supportés, avec PowerPoint hérité prévu pour l'avenir.
* Ajout du support des anciens documents Microsoft Word (*.doc)!
* Ajout du support des anciennes présentations PowerPoint (*.ppt)!
* Ajout du support des livres mobi et AZW3!
* Ajout du support des fichiers PDF marqués!
* Ajout du raccourci `ctrl+q` pour quitter l'application.
* Ajout du support des livres zippés de Bookshare (DAISY et Word)!
* Le texte alternatif pour les images intégrées devrait maintenant s'afficher correctement.
* Les documents CHM supportent maintenant correctement la navigation des liens internes.
* Correction des sons de signet se déclenchant au début du paragraphe au lieu de la position du signet.
* Correction d'aller à la page décalée de 1.
* Correction de la touche Échapper ne fonctionnant pas pour fermer la boîte de dialogue ouvrir en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applications.
* Correction du document erroné parfois reçoit le focus lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF d'images uniquement sont à nouveau détectés et vous avertissent de leur existence.
* Il est maintenant possible de naviguer à travers les images et les figures avec g/shift+g et f/shift+f, respectivement.
* Paperback respectera maintenant votre paramètre de mode sombre de l'application.
* Suppression du support DAISY XML, car il n'est plus nécessaire.
* Retour à la navigation native Win32 par première lettre dans l'arborescence de la table des matières.
* La boîte de dialogue d'erreur de chargement affiche maintenant des messages d'erreur plus détaillés.
* Le webview s'ouvrira maintenant beaucoup plus rapidement et fluidement.

### Version 0.8.2
* Ajout du support des pages aux documents RTF!
* Correction d'un bug où l'ouverture du webview dans les epubs contenant des liens externes les activerait automatiquement.
* Correction d'un bug où l'analyseur RTF ne mettrait pas d'espace entre les mots dans de rares cas.
* Correction des paragraphes étant divisés en plusieurs courtes lignes dans certains documents PDF.
* Les documents PDF ont maintenant le support basique de la navigation des liens et des titres!
* Les onglets et les sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant l'analyse PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour réouvrir le dernier document fermé.
* La boîte de dialogue Tous les documents supporte maintenant la sélection de plusieurs documents pour les ouvrir à la fois.
* Correction de quelques bugs avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (tels que le š, č, ć, ž bosniaque) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre, et espacement incorrect autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de gros fichiers.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Ajout des traductions en japonais, chinois simplifié et vietnamien!
* Ajout d'un programme de mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version!
* Ajout d'un retour audio optionnel pour atteindre un signet ou une note, merci Andre Louis pour les sons!
* Ajout du support des documents RTF!
* Ajout du support des documents DAISY XML.
* Ajout du support des fichiers de texte de document ouvert plat!
* Ajout du support des présentations de document ouvert plat!
* Ajout du support des séparateurs avec s et shift+s.
* Tout mouvement supérieur à 300 caractères ajoutera maintenant automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback à partir du plateau système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans la vue Web.
* Correction des tableaux ne s'affichant pas correctement dans les fichiers Markdown.
* Les PDF d'images uniquement vous avertissent maintenant de leur existence lorsque vous tentez d'en charger un.
* Il est maintenant possible de vérifier les nouvelles builds de développement au lieu des versions stables lors de la vérification des mises à jour.
* Incorporation correcte des informations de version dans l'exécutable Paperback.
* Fractionnement de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, menant à plus de fiabilité, de vitesse et moins de DLL.
* Réécriture de l'application entière en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout du support des tableaux pour les documents basés sur HTML et XHTML! Naviguez entre les tableaux en utilisant T et Shift+T, et appuyez sur Entrée pour en consulter un dans un webview.
* Ajout d'une fonction basique de rendu Web! Appuyez sur `Ctrl+Shift+V` pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le Web, utile pour du contenu comme la mise en forme complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov!
* Ajout d'un bouton Effacer tout à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version quand une nouvelle version est disponible.
* Correction de la restauration de la fenêtre à partir du plateau système.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la TOC dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des TOC epub vous jetant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans XML, HTML et les balises pre.
* Correction de l'erreur hors-par-un dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs de fin sur leurs lignes.
* Correction de divers problèmes d'analyseur.
* Les éléments de menu relatifs aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de document.
* Amélioration du flux de travail de traduction pour les contributeurs.
* De nombreuses réorganisations internes, déplaçant la majorité de la logique métier de l'application de C++ à Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout du support des PDF protégés par mot de passe!
* Ajout d'une très basique fonctionnalité aller à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et que cela déplace votre curseur, cette position sera maintenant mémorisée et peut être navigée avec `alt+left`/`right` arrows.
* Ajout d'une liste d'éléments! Actuellement, elle n'affiche qu'un arborescence de tous les titres de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des TOC Epub contenant des chemins relatifs.
* Correction de certains documents epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub ne s'affichant pas correctement dans la boîte de dialogue TOC.
* Correction de la possibilité d'utiliser la barre d'espace pour activer les boutons OK/annuler dans la boîte de dialogue TOC.
* Amélioration de la gestion des titres dans les documents Word.
* Vous recevrez maintenant un retour vocal si la liste des documents récents est vide lorsque vous essayez d'afficher la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu aller sous une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, cochée par défaut.
* Ajout d'une option pour faire envelopper la navigation par les éléments structurels.
* Ajout d'une option au menu Outils pour ouvrir le dossier contenant le document actuellement actif.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction basique de minuterie de sommeil, accessible avec `Ctrl+Shift+S`.
* Ajout du support pour l'analyse des livres FB2!
* Ajout du support pour l'analyse des présentations OpenDocument!
* Ajout du support pour l'analyse des fichiers OpenDocument Text!
* Les signets peuvent maintenant être placés pour marquer une ligne entière, ou pour marquer seulement du texte spécifié. Si vous n'avez aucune sélection active lors de la mise en place d'un signet, le comportement est comme pré-0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées! Naviguez entre les signets contenant des notes avec N et Shift+N, ou ouvrir la boîte de dialogue des signets avec tous les signets, uniquement les notes ou uniquement les non-notes sélectionnés avec des raccourcis spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus un préfixe « signet x » ennuyeux.
* Les livres Epub contenant du contenu HTML prétendant être du XML seront maintenant correctement manipulés.
* Correction du chargement de grands documents Markdown.
* Correction de l'appui sur la barre d'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces blancs au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne reprenant pas correctement le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus une erreur si le chargement de votre document prend plus de 5 secondes.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant correctement chargée et enregistrée.
* Il est maintenant possible de supprimer un signet directement dans la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension .paperback. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors du chargement de celui-ci, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément du menu des outils.
* Les liens à l'intérieur des documents sont maintenant entièrement supportés! Utilisez k et shift+k pour vous déplacer en avant et en arrière à travers eux, et appuyez sur Entrée pour ouvrir/activer un.
* De nombreuses réorganisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement supportée! Utilisez L et Shift+L pour aller par les listes elles-mêmes, et I et Shift+I pour parcourir les éléments de liste.
* La suppression du pavé numérique fonctionne maintenant pour supprimer les documents de la barre d'onglets en plus de la suppression normale.
* Paperback peut maintenant optionnellement se minimiser à votre plateau système! Cette option est désactivée par défaut, mais l'activation fera en sorte que l'option minimiser dans le menu système place Paperback dans votre barre d'onglets, capable d'être restaurée en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traduisible! La liste des langues qu'il supporte est actuellement assez petite, mais elle grandit constamment!
* Paperback a maintenant un site officiel, à [paperback.dev](https://paperback.dev)!
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert s'affichera maintenant dans la boîte de dialogue Infos du document.
* Le programme d'installation inclut maintenant une option pour consulter le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement étendue! Au lieu de simplement vous afficher les 10 derniers documents que vous avez ouverts, elle affiche maintenant un nombre personnalisable, les documents restants que vous avez jamais ouverts étant accessibles via une petite boîte de dialogue.
* Diverses petites améliorations apportées aux analyseurs dans l'ensemble, y compris l'insertion d'une ligne vierge entre les diapositives dans les présentations PPTX, la correction de la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word!
* Ajout du support des présentations PowerPoint!
* Correction de certains éléments de menu n'étant pas désactivés sans documents ouverts.
* Correction de l'orientation du curseur aller au pourcentage.
* Correction de la table des matières dans les livres Epub avec les chemins de fichiers codés en URL et/ou les ID de fragments.
* Correction des espaces blancs étant supprimés des titres XHTML de manière étrange.
* Correction de la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des titres de votre document, et l'affichera dans la boîte de dialogue `ctrl+t`.
* Les documents HTML auront maintenant le titre tel que défini dans la balise de titre, s'il existe. Sinon, ils continueront à utiliser le nom du fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région dynamique pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est expédiée aux côtés du programme, et plus de lecteurs d'écran seront maintenant supportés, tels que Microsoft Narrator.
* Changement de bibliothèque zip pour permettre l'ouverture d'une gamme plus large de livres epub.
* La boîte de dialogue vous demandant si vous voulez ouvrir votre document en texte brut a été complètement refaite, et elle vous permet maintenant d'ouvrir votre document en texte brut, HTML ou Markdown.
* La boîte de dialogue aller au pourcentage inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour y accéder.
* L'analyseur HTML reconnaîtra maintenant dd, dt et dl en tant qu'éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable Unicode est maintenant considéré lors de la suppression des lignes vierges.
* Vous ne serez plus invité à indiquer comment vous voulez ouvrir un fichier non reconnu à chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône de menu Démarrer optionnelle au programme d'installation.
* La table des matières devrait maintenant être plus nette dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM!
* Ajout du support des signets! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez vous déplacer en avant et en arrière à travers eux avec b et shift+b, en définir un avec `control+shift+b`, et afficher une boîte de dialogue pour accéder à un signet spécifique avec `control+b`.
* Ajout d'un programme d'installation aux côtés du fichier zip portable! Le programme d'installation installera Paperback dans votre répertoire Fichiers programme, et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec des BOM devraient maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout de beaucoup plus d'informations à la barre de statut. Il affichera maintenant votre ligne actuelle, caractère et pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne s'afficheront plus dans la sortie texte.
* Si vous passez un chemin relatif à Paperback en ligne de commande, il le résoudra maintenant correctement.
* Le mouvement au pourcentage est maintenant géré par sa propre boîte de dialogue basée sur des curseurs, accessible avec `control+shift+g`.
* Les documents sans titres ou auteurs connus auront maintenant un défaut.
* La logique de sauvegarde de position est maintenant beaucoup plus intelligente et ne devrait écrire sur le disque que si absolument nécessaire.
* Le document qui était actif lorsque vous avez fermé Paperback est maintenant mémorisé lors des redémarrages de l'application.
* L'entrée dans les boîtes de dialogue aller à la ligne et aller à la page doit maintenant être assainie plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec les manifestes codés en URL.
* Correction de la navigation des titres dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de la consommation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments TOC imbriqués dans les livres Epub plaçant votre curseur à la mauvaise position.
* Correction d'un plantage à la sortie de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne automatique!
* Il est maintenant possible de faire un don au développement de Paperback, soit par le nouvel élément de don dans le menu Aide, soit par le lien de parrainage de ce projet en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant être capable de charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Passage aux bibliothèques PDF utilisées dans Chromium, menant à une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur Supprimer sur un document dans le contrôle de l'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages au libellé de la page dans la boîte de dialogue aller à la page.
* Permettre le passage de l'onglet du contenu du document à votre liste de documents ouverts.
* Correction des raccourcis de titre ouvrant parfois les documents récents si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union logiciels inutiles de la sortie texte.
* Correction de la navigation des titres vous mettant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents Markdown!
* Ajout du support des documents PDF, y compris la capacité de naviguer entre les pages!
* Ajout de frappes de clavier pour naviguer par les titres dans le contenu HTML, y compris les livres epub et les documents Markdown. Ces frappes ont été conçues pour fonctionner de façon similaire à un lecteur d'écran.
* Correction du chargement des epubs avec les noms de fichiers codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré dedans.
* Un message est maintenant parlé si le document ne supporte pas une table des matières ou des sections, par opposition aux éléments de menu étant désactivés.
* Ajout d'un menu des documents récents! Il stocke actuellement vos 10 derniers documents ouverts, et l'appui sur Entrée sur un les ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières!
* Les documents précédemment ouverts sont maintenant mémorisés lors des redémarrages de l'application. C'est configurable par le biais du nouvel élément des options dans le menu Outils.
* Ajout de `shift+f1` pour ouvrir le fichier readme directement dans Paperback lui-même.

### Version 0.1.0
* Version initiale.
