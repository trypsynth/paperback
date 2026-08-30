<!-- machine-translated from doc/readme.md (source-hash: bdf582cc25a739ea); please review and edit as needed -->

# Paperback - version 0.9.0

## Introduction

Paperback est un lecteur d'ebooks et de documents léger, rapide et accessible, conçu pour tous, du lecteur occasionnel à l'utilisateur le plus exigeant. Il est pensé pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans superflu.

## Configuration requise

Paperback fonctionne actuellement sous Windows 10/11 et sur toutes les versions modernes de macOS ARM. Des applications natives iOS et Android sont en cours de développement actif, avec des versions de test publiques prévues peu après la sortie de la version 0.9.0 pour ordinateur, avant une version 1.0 unifiée couvrant les quatre plateformes.

## Fonctionnalités

* Entièrement autonome : aucun logiciel n'a besoin d'être installé sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface simple à onglets, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans chaque document que vous ouvrez.
* Mémorise si vous le souhaitez les documents ouverts lors de la fermeture du programme, et les restaure au lancement suivant.
* Inclut des fonctions de navigation similaires à celles du mode de navigation web de nombreux lecteurs d'écran, pour parcourir les documents rapidement et facilement.
* Inclut une boîte de dialogue de recherche complète, avec notamment un historique et la prise en charge des expressions régulières.
* Peut être utilisé de façon entièrement portable, ou installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un très grand nombre de formats de fichiers courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il existe toutefois un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous pourriez constater que les longs paragraphes sont tronqués lors du défilement vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également touchée. Il s'agit d'un bogue dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non d'un problème propre à Paperback ; un correctif a d'ailleurs mis un certain temps à émerger, compte tenu de l'enthousiasme de Vispero à répondre aux problèmes liés aux logiciels open source.

La solution de contournement, finalement apparue dans le groupe de discussion JAWS après des mois d'attente, consiste à modifier `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif au lieu d'avancer. Avec ces deux réglages en place, le défilement devrait fonctionner correctement.

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

Paperback est conçu pour une utilisation prioritairement au clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous concernent Windows. Lorsque macOS diffère, l'équivalent est indiqué entre parenthèses — principalement parce que Ctrl+G, Ctrl+W et Alt+Gauche/Droite sont déjà utilisés par d'autres conventions système ou applicatives sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : fermer tous les documents ouverts.
* `Ctrl+Shift+T` : réouvrir le dernier document fermé.
* `Ctrl+R` : afficher la boîte de dialogue « Tous les documents » (depuis les documents récents).
* `Ctrl+Q` : quitter (Windows uniquement ; sur macOS, cela se trouve dans le menu de l'application).

### Menu Aller à

* `Ctrl+F` : afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : rechercher le suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : rechercher le précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : aller au pourcentage.
* `Ctrl+P` : aller à la page (lorsque le document actuel le prend en charge).
* `=` : annoncer votre pourcentage de lecture actuel.
* `Alt+Left` (macOS : `Cmd+[`) : reculer dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : avancer dans l'historique de navigation.
* `[` : section précédente.
* `]` : section suivante.
* `Shift+H` : titre précédent.
* `H` : titre suivant.
* `Shift+1` à `Shift+6` : titre précédent de niveau 1 à 6.
* `1` à `6` : titre suivant de niveau 1 à 6.
* `Shift+P` : page précédente.
* `P` : page suivante.
* `Shift+B` : signet précédent.
* `B` : signet suivant.
* `/` : définir votre signet temporaire.
* `\` : accéder à votre signet temporaire.
* `Shift+N` : note précédente.
* `N` : note suivante.
* `Ctrl+B` : accéder à tous les signets et notes.
* `Ctrl+Alt+B` : accéder aux signets uniquement.
* `Ctrl+Alt+M` : accéder aux notes uniquement.
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Contrôle physique plutôt que Cmd) : afficher le texte de la note à la position actuelle.
* `Shift+K` : lien précédent.
* `K` : lien suivant.
* `Shift+G` : image précédente.
* `G` : image suivante.
* `Shift+F` : figure précédente.
* `F` : figure suivante.
* `Shift+T` : tableau précédent.
* `T` : tableau suivant.
* `Shift+S` : séparateur précédent.
* `S` : séparateur suivant.
* `Shift+L` : liste précédente.
* `L` : liste suivante.
* `Shift+I` : élément de liste précédent.
* `I` : élément de liste suivant.
* `Shift+,` : aller au début du conteneur actuel (liste ou tableau).
* `,` : aller au-delà de la fin du conteneur actuel (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Contrôle physique plutôt que Cmd) : afficher le nombre de mots du document actuel.
* `Ctrl+I` : afficher les informations du document.
* `Ctrl+T` : afficher la table des matières.
* `F7` : afficher la liste des éléments.
* `Ctrl+Shift+C` : ouvrir le dossier contenant.
* `Ctrl+Shift+V` : ouvrir le contenu actuel dans la vue Web.
* `Ctrl+U` : afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : importer les données du document (`.paperback`).
* `Ctrl+E` : exporter le document actuel en texte brut.
* `Ctrl+Shift+B` : activer/désactiver un signet à la sélection/au curseur actuel.
* `Ctrl+Shift+N` : ajouter ou modifier une note de signet à la sélection/au curseur actuel.
* `Ctrl+Alt+W` : activer/désactiver le retour à la ligne automatique.
* `Ctrl+Space` : lire/mettre en pause la narration audio.
* `'` : avancer dans la narration audio.
* `;` : reculer dans la narration audio.
* `Ctrl+'` : augmenter le pas de déplacement audio.
* `Ctrl+;` : diminuer le pas de déplacement audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Contrôle+Commande+F) : activer/désactiver le plein écran.
* `Ctrl+,` : ouvrir les options (macOS : Préférences, dans le menu de l'application).
* `Ctrl+Shift+S` : activer/désactiver la minuterie de veille.

### Menu Aide

* `Ctrl+F1` : afficher la boîte de dialogue À propos.
* `F1` : afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : rechercher des mises à jour.
* `Ctrl+D` : ouvrir la page de dons dans votre navigateur par défaut.

### Touches supplémentaires dans la vue document

* `Delete` / `Numpad Delete` sur le contrôle d'onglets : fermer l'onglet du document sélectionné.
* `Enter` ou `Space` dans le texte du document : activer le lien situé sous le curseur, ou ouvrir une vue de tableau lorsque le curseur est sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : ouvrir le menu contextuel.

## Langues prises en charge

Paperback est traduit dans de nombreuses langues, et d'autres sont ajoutées en permanence. Une liste complète suit ci-dessous.

Pour savoir comment contribuer, veuillez lire notre [Guide de traduction](translating.md).

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

### Dons
Les personnes suivantes ont fait des dons d'un certain montant au développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici ; je n'ajoute que les personnes qui souhaitent rendre leur don public.

Remarque : je considère qu'un parrainage public sur GitHub justifie une inclusion automatique dans cette liste.

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

### Version 0.9.0

#### Ajouté

##### Général
* Un outil en ligne de commande, appelé pb, pour convertir rapidement n'importe quel format pris en charge par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte des documents est désormais paginé, ce qui signifie que vous pouvez charger des livres de dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute anomalie constatée à ce sujet.

##### Prise en charge des plateformes
* Prise en charge de Windows ARM64 !
* Prise en charge native de macOS !
* Un basculement en plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton Localiser pour retrouver les livres manquants dont le chemin vient de changer.
* Un filtre d'état et une barre d'état, afin que vous puissiez filtrer par état de document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet Lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé depuis Général) ;
    * Afficher les tableaux en ligne (nouveauté de cette version, voir ci-dessous) ;
    * Police ;
    * Couleur d'arrière-plan ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu pour le retour à la ligne automatique et le raccourci clavier correspondant.
* Un basculement pour déterminer comment vous souhaitez que les tableaux soient affichés, et unification de l'affichage des tableaux entre les documents.

##### Navigation
* Prise en charge de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, semblable au mode navigation des lecteurs d'écran.
* Le raccourci clavier signe égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils sont conservés. Utilisez la barre oblique pour en placer un et la barre oblique inverse pour y accéder.

##### Nombre de mots
* Temps de lecture estimé dans la boîte de dialogue du nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette mesure réellement utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue du nombre de mots, le nombre de mots sélectionnés sera désormais affiché.

##### Raccourcis clavier
* La possibilité de personnaliser chaque raccourci clavier de l'application via une simple boîte de dialogue.
* Un raccourci clavier configurable pour restaurer Paperback depuis la zone de notification.

##### Langues
* Néerlandais, finnois et polonais.

##### Exportation
* Extension de l'élément de menu Exporter pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Programme de mise à jour
* Un bouton Annuler dans la boîte de dialogue de mise à jour en cours.
* Le programme de mise à jour vérifie désormais que le fichier téléchargé n'a pas été altéré.

##### Vue Web
* La vue Web s'ouvre désormais à votre position de lecture actuelle.

##### Livres DAISY
* Prise en charge des livres DAISY 2.0.
* Prise en charge de la lecture audio DAISY 2.02.

##### Livres audio
* La possibilité de lire des livres audio, prenant actuellement en charge à la fois l'audio DAISY (y compris DAISY audio + texte) et les archives zip de fichiers audio.
* Des raccourcis clavier et éléments de menu pour lire/mettre en pause la narration, avancer et reculer, et ajuster la durée du déplacement.
* Des options pour synchroniser le curseur de lecture avec la lecture audio, définir la durée du déplacement audio, et choisir si le déplacement au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Prise en charge des listes, des éléments de liste, des figures et des images.

##### PowerPoint
* Les documents PowerPoint prennent désormais en charge les tableaux.

#### Corrigé

##### Général
* Les documents encodés dans des encodages CJK anciens, tels que GBK, Big5 et Shift_JIS, s'affichent désormais correctement au lieu d'un amas de mojibake.
* « Réouvrir le dernier document fermé » qui tentait de réouvrir le fichier readme intégré.
* L'onglet sélectionné qui ne recevait pas correctement le focus après le redémarrage de Paperback.
* La gestion par Paperback des fichiers sur les lecteurs réseau Windows : appuyer sur Afficher le fichier dans le dossier met désormais correctement le focus sur le fichier stocké sur le réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus chargés de force lors de la restauration des documents ; à la place, une confirmation vous sera demandée lorsqu'un tel fichier est trouvé.
* Ouvrir le dossier contenant met désormais le focus sur le fichier concerné dans l'explorateur.
* L'ouverture du fichier readme respecte désormais la langue que vous avez sélectionnée.
* L'interface utilisateur de Paperback s'adapte désormais correctement aux écrans à haute résolution.
* Le menu se met désormais correctement à jour, et le focus se déplace vers le contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode de communication interprocessus (IPC) beaucoup plus sécurisée sous Windows.
* Le titre du document actif est désormais lu lors du changement d'onglet.
* Réduction de l'utilisation de la mémoire sur les documents volumineux en divisant par deux la taille des tables d'index internes par caractère.

##### Boîte de dialogue Tous les documents
* La touche Échap qui ne fermait pas les boîtes de dialogue Informations sur le document et Tous les documents.
* La barre de titre qui ne se mettait pas à jour après la fermeture d'un document depuis la boîte de dialogue Tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lorsqu'il est ouvert via Shift+F1.
* La suppression de documents depuis la boîte de dialogue des documents récents fermera désormais également leur onglet actif.
* Votre filtre de recherche est désormais conservé après la suppression d'un document.

##### Navigation
* La navigation par page qui annonçait un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage qui plaçaient votre curseur à la mauvaise position dans les documents volumineux.
* Rechercher et Rechercher le suivant qui ne respectaient pas la fenêtre du document chargé dans les documents volumineux.

##### Signets
* Les sons de signet/note devraient désormais être joués exclusivement lorsque vous naviguez sur un mot qui en contient un.

##### Lisibilité
* L'application du retour à la ligne automatique qui vous renvoyait au début de votre document.

##### Vue Web
* La boîte de dialogue de la vue Web qui n'était pas redimensionnable et s'affichait à une taille initiale très réduite.
* Les images devraient désormais s'afficher correctement dans la vue Web intégrée.

##### Programme de mise à jour
* Le programme de mise à jour affiche désormais correctement le contenu des balises de code Markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY qui affichaient des informations incorrectes dans la barre d'état.
* Le chargement de livres DAISY comportant des déclarations d'encodage erronées.

##### Documents RTF
* L'analyse des documents RTF contenant des caractères non latins.
* Les groupes `\pict` RTF, afin que les données d'image intégrées ne s'infiltrent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi qui divisaient les balises HTML et inséraient des données parasites dans le texte du livre.
* Les liens dans les anciens livres Mobi.
* Amélioration majeure de l'analyse AZW3.

##### Documents Word
* Les documents Word avec des noms de styles spécifiques à une langue qui n'affichaient pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd qui ne produisaient pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient désormais à l'extraction en texte brut pour les PDF faussement balisés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne feront plus planter Paperback à l'ouverture.

### Version 0.8.5
* Ajout de la prise en charge des pages dans les livres epub.
* Ajout de la prise en charge des documents Microsoft Office chiffrés. Actuellement, l'ancien format Word, le Word moderne et le Powerpoint moderne sont pris en charge, l'ancien Powerpoint étant prévu pour l'avenir.
* Ajout de la prise en charge des anciens documents Microsoft Word (*.doc) !
* Ajout de la prise en charge des anciennes présentations Powerpoint (*.ppt) !
* Ajout de la prise en charge des livres mobi et AZW3 !
* Ajout de la prise en charge des fichiers PDF balisés !
* Ajout du raccourci ctrl+q pour quitter l'application.
* Ajout de la prise en charge des livres zippés de Bookshare (DAISY et Word) !
* Le texte alternatif des images intégrées devrait désormais être correctement affiché.
* Les documents CHM prennent désormais correctement en charge la navigation par liens internes.
* Correction des sons de signet qui se déclenchaient au début du paragraphe au lieu de la position du signet.
* Correction de la fonction Aller à la page qui était décalée de 1.
* Correction de la touche Échap qui ne fonctionnait pas pour fermer la boîte de dialogue Ouvrir en tant que.
* Correction du menu contextuel du lecteur qui ne s'affichait pas au clic droit ou avec la touche Applications.
* Correction du mauvais document qui recevait parfois le focus lors de l'ouverture de documents depuis la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous en êtes averti.
* Il est désormais possible de naviguer parmi les images et les figures avec g/shift+g et f/shift+f, respectivement.
* Paperback respecte désormais votre paramètre de mode sombre de l'application.
* Suppression de la prise en charge de DAISY XML, car elle n'est plus nécessaire.
* Retour à la navigation native Win32 par première lettre dans l'arborescence de la table des matières.
* La boîte de dialogue d'erreur de chargement affiche désormais des messages d'erreur plus détaillés.
* La vue Web s'ouvre désormais beaucoup plus rapidement et de manière plus fluide.

### Version 0.8.2
* Ajout de la prise en charge des pages dans les documents RTF !
* Correction d'un bug où l'ouverture de la vue Web dans les epubs contenant des liens externes les activait automatiquement.
* Correction d'un bug où l'analyseur RTF n'insérait pas d'espace entre les mots dans de rares cas.
* Correction des paragraphes divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF disposent désormais d'une prise en charge de base de la navigation par liens et par titres !
* Les tabulations et les sauts de ligne RTF sont désormais affichés exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque éprouvée pdfium pour l'analyse des PDF, rendant à nouveau le rendu des PDF beaucoup plus fiable.

### Version 0.8.1
* Ajout de Ctrl+Shift+T pour réouvrir le dernier document fermé.
* La boîte de dialogue Tous les documents prend désormais en charge la sélection de plusieurs documents à ouvrir en même temps.
* Correction de quelques bugs de l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non ASCII (comme les š, č, ć, ž bosniens) qui étaient corrompus lors de l'ouverture d'un fichier via une seconde instance de Paperback.
* Correction du texte PDF lu dans le mauvais ordre, et des espacements incorrects autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Ajout des traductions japonaise, chinoise simplifiée et vietnamienne !
* Ajout d'un programme de mise à jour automatique qui remplace désormais votre version installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout d'un retour sonore facultatif lorsque vous atteignez un signet ou une note, merci à Andre Louis pour les sons !
* Ajout de la prise en charge des documents RTF !
* Ajout de la prise en charge des documents DAISY XML.
* Ajout de la prise en charge des fichiers Flat Open Document Text !
* Ajout de la prise en charge des présentations Flat Open Document !
* Ajout de la prise en charge des séparateurs avec s et shift+s.
* Tout déplacement de plus de 300 caractères sera désormais automatiquement ajouté à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis la zone de notification.
* Correction des documents Markdown qui affichaient du texte brut au lieu du HTML rendu dans la vue Web.
* Correction des tableaux qui ne s'affichaient pas correctement dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertiront désormais de leur nature lorsque vous tentez d'en charger un.
* Il est désormais possible de rechercher de nouvelles versions de développement au lieu des versions stables lors de la recherche de mises à jour.
* Intégration correcte des informations de version dans l'exécutable de Paperback.
* Division de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, apportant plus de fiabilité, de rapidité et moins de DLL.
* Réécriture complète de l'application en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement, et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura désormais des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout de la prise en charge des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux avec T et Shift+T, et appuyez sur Entrée pour en afficher un dans une vue Web.
* Ajout d'une fonction de rendu Web de base ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un moteur de rendu Web, utile pour du contenu comme une mise en forme complexe ou des exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Tout effacer à la boîte de dialogue Tous les documents.
* Le vérificateur de mises à jour affiche désormais les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis la zone de notification.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la table des matières dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant portant la même lettre dans la table des matières.
* Correction de la boîte de dialogue de recherche qui ne se masquait pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des tables des matières d'epub qui vous renvoyaient parfois au mauvais élément.
* Correction de divers problèmes de gestion des espaces dans les balises XML, HTML et pre.
* Correction d'une erreur de décalage d'une unité dans la navigation par liens.
* Correction de certains livres qui présentaient des espaces en fin de ligne.
* Correction de divers problèmes des analyseurs.
* Les éléments de menu liés aux signets ainsi que la liste des éléments sont désormais correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de documents.
* Amélioration du processus de traduction pour les contributeurs.
* Nombreuses refontes internes, déplaçant la majorité de la logique métier de l'application du C++ vers Rust pour de meilleures performances et une meilleure maintenabilité.

### Version 0.6.1
* Ajout de la prise en charge des PDF protégés par mot de passe !
* Ajout d'une fonction très basique d'accès à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et que cela déplace votre curseur, cette position sera désormais mémorisée et accessible avec alt+flèches gauche/droite.
* Ajout d'une liste d'éléments ! Actuellement, elle n'affiche qu'une arborescence de tous les titres de votre document ou une liste de liens, mais il est prévu de l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens qui ne fonctionnaient pas correctement dans certains documents Epub.
* Correction de l'analyse des tables des matières Epub contenant des chemins relatifs.
* Correction de certains documents epub qui n'affichaient pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub qui ne s'affichaient pas correctement dans la boîte de dialogue de la table des matières.
* Correction de l'impossibilité d'utiliser la barre d'espace pour activer les boutons OK/Annuler dans la boîte de dialogue de la table des matières.
* Amélioration de la gestion des titres dans les documents Word.
* Vous recevrez désormais un retour vocal si la liste des documents récents est vide lorsque vous tentez d'ouvrir la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu Aller sous une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, cochée par défaut.
* Ajout d'une option pour rendre cyclique la navigation par éléments structurels.
* Ajout d'une option au menu Outils pour ouvrir le dossier contenant le document actuellement actif.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction basique de minuteur de mise en veille, accessible avec Ctrl+Shift+S.
* Ajout de la prise en charge de l'analyse des ebooks FB2 !
* Ajout de la prise en charge de l'analyse des présentations OpenDocument !
* Ajout de la prise en charge de l'analyse des fichiers OpenDocument Text !
* Les signets peuvent désormais marquer une ligne entière, ou uniquement un texte spécifié. Si aucune sélection n'est active lors du placement d'un signet, le comportement est identique à celui d'avant la version 0.6, et la ligne entière sera marquée. En revanche, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent désormais comporter des notes textuelles facultatives ! Naviguez entre les signets contenant des notes avec N et Shift+N, ou ouvrez la boîte de dialogue des signets avec tous les signets, uniquement les notes, ou uniquement les non-notes sélectionnés grâce à des raccourcis spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus l'agaçant préfixe « signet x ».
* Les livres Epub contenant du contenu HTML se faisant passer pour du XML seront désormais traités correctement.
* Correction du chargement des documents Markdown volumineux.
* Correction de l'appui sur la barre d'espace dans l'arborescence de la table des matières qui activait le bouton OK.
* Correction de la gestion des espaces au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte qui ne récupérait parfois pas le focus lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue Aller au pourcentage qui ne mettait pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera désormais rendu correctement.
* Si vous chargez un livre via un paramètre de ligne de commande alors qu'une instance de Paperback est déjà en cours d'exécution, vous n'obtiendrez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si Paperback est exécuté en tant qu'administrateur, la configuration sera désormais correctement chargée et enregistrée.
* Il est désormais possible de supprimer un signet directement depuis la boîte de dialogue des signets.
* Il est désormais possible d'importer et d'exporter vos signets et votre position de lecture pour un document donné. Le fichier généré porte le nom du fichier avec l'extension .paperback. Si un tel fichier est trouvé dans le même dossier qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement à l'aide d'un élément du menu Outils.
* Les liens à l'intérieur des documents sont désormais entièrement pris en charge ! Utilisez k et shift+k pour vous déplacer vers l'avant et vers l'arrière, et appuyez sur Entrée pour en ouvrir/activer un.
* Nombreuses refontes internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est désormais prétraité afin d'être conforme à CommonMark avant le rendu.
* La navigation par listes et par éléments de liste est désormais entièrement prise en charge ! Utilisez L et Shift+L pour parcourir les listes elles-mêmes, et I et Shift+I pour parcourir les éléments de liste.
* La touche Suppr du pavé numérique fonctionne désormais pour supprimer des documents de la barre d'onglets, en plus de la touche Suppr normale.
* Paperback peut désormais éventuellement se réduire dans votre zone de notification ! Cette option est désactivée par défaut, mais son activation fera que l'option Réduire du menu système placera Paperback dans votre zone de notification, d'où il pourra être restauré en cliquant sur l'icône créée.
* Paperback est désormais entièrement traduisible ! La liste des langues prises en charge est actuellement assez restreinte, mais elle ne cesse de s'allonger !
* Paperback a désormais un site officiel, à l'adresse [paperback.dev](https://paperback.dev) !
* Les documents PPTX afficheront désormais une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert sera désormais affiché dans la boîte de dialogue des informations sur le document.
* Le programme d'installation inclut désormais une option pour consulter le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement étendue ! Au lieu de vous montrer simplement les 10 derniers documents ouverts, elle affichera désormais un nombre personnalisable, le reste des documents que vous avez déjà ouverts étant accessible via une petite boîte de dialogue.
* Diverses petites améliorations des analyseurs dans l'ensemble, notamment l'insertion d'une ligne vide entre les diapositives des présentations PPTX, la correction de la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout de la prise en charge des documents Microsoft Word !
* Ajout de la prise en charge des présentations PowerPoint !
* Correction de certains éléments de menu qui n'étaient pas désactivés lorsqu'aucun document n'était ouvert.
* Correction de l'orientation du curseur de la boîte de dialogue Aller au pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichiers et/ou des identifiants de fragment encodés en URL.
* Correction des espaces supprimés de manière étrange dans les titres XHTML.
* Correction de la gestion des espaces à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown prennent désormais en charge la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des titres de votre document, et vous l'affichera dans la boîte de dialogue ctrl+t.
* Les documents HTML auront désormais le titre défini dans la balise title, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage d'UniversalSpeech à l'utilisation d'une région live pour rapporter la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est plus livrée avec le programme, et que davantage de lecteurs d'écran seront désormais pris en charge, comme Microsoft Narrator.
* Changement de bibliothèque zip pour permettre l'ouverture d'un plus grand nombre de livres epub.
* La boîte de dialogue vous demandant si vous souhaitez ouvrir votre document en texte brut a été entièrement refaite, et elle vous permet désormais d'ouvrir votre document en texte brut, HTML ou Markdown.
* La boîte de dialogue Aller au pourcentage comprend désormais un champ de texte vous permettant de saisir manuellement un pourcentage vers lequel accéder.
* L'analyseur HTML reconnaîtra désormais dd, dt et dl comme des éléments de liste.
* La table des matières des livres Epub sera à nouveau préservée à l'identique.
* L'espace insécable Unicode est désormais pris en compte lors de la suppression des lignes vides.
* On ne vous demandera plus comment vous voulez ouvrir un fichier non reconnu à chaque chargement, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône facultative dans le menu Démarrer au programme d'installation.
* La table des matières devrait désormais être plus propre dans quelques cas, par exemple si vous avez un élément parent et un élément enfant avec le même texte à la même position, vous ne verrez plus que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 contenant des chemins absolus.
* Les documents CHM devraient désormais afficher leur titre tel qu'il est défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout de la prise en charge des fichiers CHM !
* Ajout de la prise en charge des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous voulez. Vous pouvez les parcourir vers l'avant et vers l'arrière avec b et shift+b, en placer un avec control+shift+b, et ouvrir une boîte de dialogue pour accéder à un signet spécifique avec control+b.
* Ajout d'un programme d'installation en plus du fichier zip portable ! Le programme d'installation installera Paperback dans votre dossier Program Files, et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte comportant des BOM devraient désormais être décodés correctement, et le BOM ne sera plus affiché au début du texte.
* Ajout de bien plus d'informations à la barre d'état. Elle affichera désormais votre ligne, votre caractère et votre pourcentage de lecture actuels.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne seront plus affichés dans la sortie texte.
* Si vous transmettez un chemin relatif à Paperback en ligne de commande, il sera désormais résolu correctement.
* Le déplacement par pourcentage est désormais géré par sa propre boîte de dialogue à curseur, accessible avec control+shift+g.
* Les documents sans titre ou auteur connus auront désormais toujours une valeur par défaut.
* La logique d'enregistrement de la position est désormais bien plus intelligente et ne devrait écrire sur le disque que lorsque c'est absolument nécessaire.
* Le document qui était actif lors de la fermeture de Paperback est désormais mémorisé entre les redémarrages de l'application.
* La saisie dans les boîtes de dialogue Aller à la ligne et Aller à la page devrait désormais être validée plus strictement.
* Correction de la navigation dans la table des matières des livres epub 3 comportant des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes encodés en URL.
* Correction de la navigation par titres dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction d'une utilisation élevée du processeur dans les documents aux titres longs, due à une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments imbriqués de la table des matières dans les livres Epub qui plaçaient votre curseur à la mauvaise position.
* Correction d'un plantage à la fermeture de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est désormais possible de faire un don pour le développement de Paperback, soit via le nouvel élément Faire un don du menu Aide, soit via le lien « sponsor this project » au bas de la page principale du dépôt GitHub.
* Les documents Markdown auront désormais toujours un titre, et Paperback devrait maintenant pouvoir charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront désormais toujours un titre, même si les métadonnées sont absentes.
* Passage à la bibliothèque PDF utilisée dans Chromium, ce qui rend l'analyse des PDF bien plus fiable dans l'ensemble.
* Vous ne pouvez désormais avoir qu'une seule instance de Paperback en cours d'exécution à la fois. Exécuter paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà lancée.
* Vous pouvez désormais appuyer sur Suppr sur un document dans le contrôle d'onglets pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue Aller à la page.
* Autorisation de la tabulation depuis le contenu du document vers votre liste de documents ouverts.
* Correction des raccourcis de titres qui ouvraient parfois les documents récents si vous en aviez suffisamment.
* Paperback supprimera désormais les traits d'union conditionnels inutiles de la sortie texte.
* Correction de la navigation par titres qui vous plaçait parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout de la prise en charge des documents markdown !
* Ajout de la prise en charge des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis pour naviguer par titres dans le contenu HTML, y compris les livres epub et les documents markdown. Ces raccourcis ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epubs comportant des noms de fichiers encodés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 contenant du XHTML intégré.
* Un message est désormais énoncé si le document ne prend pas en charge une table des matières ou des sections, au lieu que les éléments de menu soient désactivés.
* Ajout d'un menu des documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue de recherche, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et la prise en charge des expressions régulières !
* Les documents précédemment ouverts sont désormais mémorisés entre les redémarrages de l'application. Ceci est configurable via le nouvel élément Options du menu Outils.
* Ajout de shift+f1 pour ouvrir le fichier readme directement dans Paperback.

### Version 0.1.0
* Version initiale.
