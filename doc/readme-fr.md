<!-- machine-translated from doc/readme.md (source-hash: 197dbd0c570ba62e); please review and edit as needed -->

# Livre de poche - version 0.8.5 {#paperback---version-0.8.5}

## Introduction

Paperback est un lecteur de livres électroniques et de documents léger,
rapide et accessible destiné à tous, des lecteurs occasionnels aux
utilisateurs intensifs. Il est conçu pour être compatible avec les
lecteurs d\'écran, offrir des vitesses élevées et une expérience
d\'utilisation épurée.

## Configuration requise {#system-requirements}

Paperback fonctionne actuellement sous Windows, macOS, iOS et Android.

## Fonctionnalités {#features}

-   Entièrement autonome, ne nécessitant l'installation d'aucun logiciel
    sur votre ordinateur pour commencer à lire.
-   Incroyablement rapide, même sur du matériel ancien.
-   Interface simple à onglets, vous permettant d'ouvrir autant de
    documents que vous le souhaitez côte à côte.
-   Enregistre votre position exacte de lecture dans chaque document que
    vous ouvrez.
-   En option, il mémorise les documents que vous aviez ouverts lorsque
    vous avez fermé le programme, et les restaure au prochain lancement.
-   Intègre des fonctionnalités de navigation similaires à celles du
    mode de navigation Web de nombreux lecteurs d'écran, pour parcourir
    rapidement et facilement les documents.
-   Comprend une boîte de dialogue de recherche robuste, avec des
    fonctionnalités telles que l\'historique et la prise en charge des
    expressions régulières.
-   Peut être exécuté de manière entièrement portable, ou installé avec
    des associations de fichiers configurées automatiquement.
-   Prend en charge une vaste gamme de formats de fichiers courants.

## Compatibilité avec les lecteurs d\'écran {#screen-reader-compatibility}

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il
existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et les afficheurs braille {#jaws-and-braille-displays}

Si vous utilisez JAWS avec un afficheur braille, vous constaterez
peut-être que les longs paragraphes sont tronqués lorsque vous faites
défiler le texte vers l'avant à l'aide des touches de navigation de
votre afficheur. La commande « Lire le paragraphe actuel » est également
affectée. Il s'agit d'un bug dans la gestion par JAWS du contrôle de
texte RICHEDIT50W, et non d'un problème inhérent à Paperback lui-même ;
il a d'ailleurs fallu un certain temps pour qu'un correctif soit
proposé, compte tenu de l'enthousiasme dont fait preuve Vispero pour
répondre aux problèmes liés aux logiciels libres.

La solution de contournement, finalement proposée par le groupe de
discussion JAWS après des mois d'attente, consiste à modifier
`paperback.jcf` et de définir « Présentation et défilement en braille »
sur « Toujours utiliser le DOM si disponible ». Vous devrez également
activer « Défilement du texte par paragraphe », sinon votre écran
restera sur le paragraphe actif au lieu d'avancer. Une fois ces deux
paramètres configurés, le défilement devrait fonctionner correctement.

## Types de fichiers actuellement pris en charge {#currently-supported-file-types}

Paperback prend en charge les formats et extensions suivants :

-   Fichiers d'aide CHM (`.chm`)
-   livres DAISY (`.opf`, `.zip`)
-   Livres EPUB (`.epub`)
-   livres électroniques FB2 (`.fb2`)
-   Documents HTML (`.htm`, `.html`, `.xhtml`)
-   Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
-   Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
-   Présentations OpenDocument (`.odp`, `.fodp`)
-   Fichiers texte OpenDocument (`.odt`, `.fodt`)
-   Documents PDF (`.pdf`)
-   Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
-   Documents RTF (`.rtf`)
-   Fichiers texte brut et fichiers journaux (`.txt`, `.log`)

## Raccourcis clavier {#keyboard-shortcuts}

Paperback est conçu pour une utilisation privilégiant le clavier. Voici
les raccourcis actuels.

Les raccourcis ci-dessous s'appliquent à Windows. Lorsque macOS présente
des différences, l'équivalent est indiqué entre parenthèses ---
principalement parce que Ctrl+G, Ctrl+W et Alt+Flèche gauche/droite sont
déjà utilisés par d'autres conventions du système ou des applications
sur cette plateforme.

### Menu Fichier {#file-menu}

-   `Ctrl+O`: Ouvrir un document.
-   `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
-   `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents
    ouverts.
-   `Ctrl+Shift+T`: Rouvrir le dernier document fermé.
-   `Ctrl+R`: Afficher la boîte de dialogue « Tous les documents » (à
    partir de « Documents récents » ).
-   `Ctrl+Q`: Quitter (Windows uniquement ; sous macOS, cette option se
    trouve dans le menu de l'application).

### Menu Aller {#go-menu}

-   `Ctrl+F`: Afficher la boîte de dialogue « Rechercher ».
-   `F3` (macOS : `Cmd+G`) : Rechercher le suivant.
-   `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher le précédent.
-   `Ctrl+G` (macOS : `Cmd+L`): Aller à la ligne.
-   `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`): Aller au pourcentage.
-   `Ctrl+P`: Aller à la page (si cette fonctionnalité est prise en
    charge par le document actuel).
-   `Alt+Left` (macOS : `Cmd+[`) : Reculer dans l' historique de
    navigation.
-   `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l' historique de
    navigation.
-   `[`: Section précédente.
-   `]`: Section suivante.
-   `Shift+H`: Rubrique précédente.
-   `H`: Rubrique suivante.
-   `Shift+1` jusqu'à `Shift+6`: Rubrique précédente aux niveaux 1 à 6.
-   `1` jusqu'à `6`: Rubrique suivante de niveau 1 à 6.
-   `Shift+P`: Page précédente.
-   `P`: Page suivante.
-   `Shift+B`: Signet précédent.
-   `B`: Signet suivant.
-   `Shift+N`: Note précédente.
-   `N`: Note suivante.
-   `Ctrl+B`: Accéder à tous les signets et toutes les notes.
-   `Ctrl+Alt+B`: Accéder uniquement aux signets.
-   `Ctrl+Alt+M`: Accéder uniquement aux notes.
-   `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c\'est-à-dire la touche
    physique « Contrôle » plutôt que « Cmd ») : Afficher le texte de la
    note à la position actuelle.
-   `Shift+K`: Lien précédent.
-   `K`: Lien suivant.
-   `Shift+G`: Image précédente.
-   `G`: Image suivante.
-   `Shift+F`: Figure précédente.
-   `F`: Figure suivante.
-   `Shift+T`: Tableau précédent.
-   `T`: Tableau suivant.
-   `Shift+S`: Séparateur précédent.
-   `S`: Séparateur suivant.
-   `Shift+L`: Liste précédente.
-   `L`: Liste suivante.
-   `Shift+I`: Élément de liste précédent.
-   `I`: Élément suivant de la liste.
-   `Shift+,`: Aller au début du conteneur actuel (liste ou tableau).
-   `,`: Aller au-delà de la fin du conteneur actuel (liste ou tableau).

### Menu Outils {#tools-menu}

-   `Ctrl+W` (macOS : `RawCtrl+W`, c\'est-à-dire la touche Contrôle
    physique plutôt que Cmd) : Afficher le nombre de mots du document
    actuel.
-   `Ctrl+I`: Afficher les informations sur le document.
-   `Ctrl+T`: Afficher la table des matières.
-   `F7`: Afficher la liste des éléments.
-   `Ctrl+Shift+C`: Ouvrir le dossier contenant le document.
-   `Ctrl+Shift+V`: Ouvrir le contenu actuel dans la vue Web.
-   `Ctrl+U`: Afficher le code source du document dans un nouvel onglet.
-   `Ctrl+Shift+E`: Exporter les données du document (`.paperback`).
-   `Ctrl+Shift+I`: Importer les données du document (`.paperback`).
-   `Ctrl+E`: Exporter le document actuel au format texte brut.
-   `Ctrl+Shift+B`: Ajouter ou supprimer un signet à l\'emplacement de
    la sélection/du curseur.
-   `Ctrl+Shift+N`: Ajouter ou modifier une note de signet à
    l\'emplacement de la sélection/du curseur.
-   `Ctrl+Alt+W`: Activer/désactiver le retour à la ligne automatique.
-   `Ctrl+,`: Ouvrir les options (macOS : Préférences, dans le menu de
    l\'application ).
-   `Ctrl+Shift+S`: Activer/désactiver la minuterie de mise en veille.

### Menu Aide {#help-menu}

-   `Ctrl+F1`: Afficher la boîte de dialogue « À propos ».
-   `F1`: Afficher l'aide dans votre navigateur par défaut.
-   `Shift+F1`: Afficher l\'aide dans Paperback.
-   `Ctrl+Shift+U`: Rechercher les mises à jour.
-   `Ctrl+D`: Ouvrir la page de dons dans votre navigateur par défaut.

### Touches supplémentaires pour l\'affichage des documents : {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` dans le contrôle des onglets : Fermer
    l\'onglet du document sélectionné.
-   `Enter` ou `Space` dans le texte du document : Activer le lien situé
    au niveau du curseur, ou ouvrir une vue de tableau lorsque le
    curseur se trouve sur un marqueur de tableau.
-   `Shift+F10` ou la touche Menu/Application dans le texte du document
    : ouvre le menu contextuel.

## Langues prises en charge {#supported-languages}

Paperback est traduit dans de nombreuses langues, et d'autres sont
ajoutées en permanence. Vous trouverez ci-dessous la liste complète.

Pour savoir comment contribuer, veuillez consulter notre [guide de
traduction](translating.md).

-   Bosniaque
-   Tchèque
-   Néerlandais
-   Finnois
-   Français
-   Allemand
-   Japonais
-   Polonais
-   Portugais (Brésil)
-   Russe
-   Chinois simplifié
-   Serbe
-   Espagnol
-   Vietnamien

## Crédits {#credits}

### Développement {#development}

-   Quin Gillespie : développeur principal et fondateur du projet.
-   Aryan Choudhary : contributeur principal.

### Dons {#donations}

Les personnes suivantes ont fait des dons d\'un montant significatif
pour le développement de Paperback. Si vous faites un don, votre nom ne
sera pas automatiquement ajouté ici ; je n\'ajoute que les personnes qui
souhaitent que leur don soit rendu public.

Remarque : je considère qu\'un parrainage GitHub public constitue un
motif suffisant pour une inclusion automatique dans cette liste.

-   Alex Hall
-   Brandon McGinty
-   Brian Hartgen
-   Debbie Yuille
-   Devin Prater
-   Félix Steindorff
-   Hamish Mackenzie
-   James Scholes
-   Jayson Smith
-   Jonathan Rodriguez
-   Jonathan Schuster
-   Keao Wright
-   Michael Marshall
-   Pratik Patel
-   Roberto Perez
-   Sean Randall
-   Timothy Wynn
-   Tyler Rodick

## Journal des modifications {#changelog}

### Version 0.9.0 (non publiée) {#version-0.9.0-unreleased}

-   Ajout d\'un bouton « Annuler » à la boîte de dialogue « Mise à jour
    en cours ».
-   Ajout d'un outil en ligne de commande, appelé « pb », permettant de
    convertir rapidement n'importe quel format pris en charge par
    Paperback en HTML, Markdown ou texte brut.
-   Ajout d'un raccourci clavier configurable pour restaurer Paperback
    depuis la barre d'état système.
-   Ajout d'un bouton « Localiser » dans la boîte de dialogue « Tous les
    documents » pour localiser les livres manquants dont le chemin
    d'accès vient de changer.
-   Ajout de la touche = pour afficher le pourcentage de lecture actuel
    d'un livre.
-   Ajout d'une option permettant de déplacer le curseur de texte au
    début de la ligne lors de la navigation, à l'instar du mode «
    parcourir » de certains lecteurs d'écran.
-   Ajout d'un onglet « Lisibilité » dans la boîte de dialogue des
    options, proposant les options suivantes :
    -   Retour à la ligne automatique (déplacé depuis l\'onglet «
        Général ») ;
    -   Affichage des tableaux en ligne (nouveauté de cette version,
        voir ci-dessous) ;
    -   Police ;
    -   Couleur d\'arrière-plan ;
    -   Interligne ;
    -   Espacement des paragraphes ;
    -   Espacement des lettres ;
    -   Alignement du texte.
-   Ajout d\'un bouton permettant de choisir le mode d\'affichage des
    tableaux, et harmonisation de l\'affichage des tableaux dans tous
    les documents.
-   Ajout d'une option « Afficher la source » permettant d'ouvrir le
    code source d'un document dans un nouvel onglet, ce qui est utile
    pour modifier du Markdown, par exemple.
-   Ajout d'un élément de menu « Retour à la ligne automatique » et d'un
    raccourci clavier correspondant.
-   Ajout de la durée de lecture estimée dans la boîte de dialogue de
    comptage de mots, ainsi que la possibilité de définir votre vitesse
    de lecture pour que cette mesure soit réellement utile.
-   Prise en charge de Windows sur ARM64 ajoutée !
-   Ajout de la prise en charge d'Android !
-   Ajout de la prise en charge d'iOS !
-   Prise en charge de macOS ajoutée !
-   Ajout de nouvelles langues : néerlandais, finnois et polonais.
-   Ajout de la prise en charge de la navigation par conteneur.
-   Ajout de la prise en charge des listes, des éléments de liste, des
    figures et des images dans les documents CHM .
-   Ajout de signets temporaires : appuyez sur la barre oblique pour en
    créer un, sur la barre oblique inversée pour y accéder.
-   Les sons des signets/notes devraient désormais être lus correctement
    et exclusivement lorsque vous naviguez sur un mot en contenant un.
-   Les documents encodés dans des encodages CJK hérités, tels que GBK,
    Big5 et Shift_JIS, s\'affichent désormais correctement au lieu
    d\'apparaître sous forme d\'une série de caractères illisibles.
-   Les documents dont le contenu change sur le disque peuvent
    désormais, si vous le souhaitez, être rechargés automatiquement avec
    le nouveau contenu.
-   L\'option du menu d\'exportation a été étendue pour permettre
    l\'exportation au format HTML et Markdown en plus du texte brut.
-   Correction d'un problème où l'application du retour à la ligne
    automatique vous renvoyait au début de votre document.
-   Correction de l'affichage d'informations erronées dans la barre
    d'état des livres Daisy.
-   Correction des éléments dl, dt et dd qui ne généraient pas de sauts
    de ligne dans les documents XHTML .
-   Correction d'un problème où la touche Échap ne fermait pas les
    boîtes de dialogue « Informations sur le document » et « Tous les
    documents ». Correction d'un problème où les ancres \`filepos\` dans
    les livres au format Mobi séparaient les balises HTML et inséraient
    des caractères indésirables dans le texte du livre.
-   Correction d'un problème où les ancres filepos dans les livres Mobi
    divisaient les balises HTML et inséraient des caractères
    indésirables dans le texte du livre.
-   Correction du ralentissement observé à l\'approche de la fin du
    champ de texte dans les documents volumineux. Correction des liens
    dans les livres Mobi hérités.
-   Correction des liens dans les livres Mobi hérités.
-   Correction du chargement des livres DAISY comportant des
    déclarations d'encodage erronées.
-   Correction d\'un problème de navigation entre les pages annonçant
    une ligne de texte incorrecte dans certaines situations.
-   Correction de l\'analyse des documents RTF contenant des caractères
    non latins et des échappements Unicode « ? ». Correction de
    l\'erreur de l\'application qui tentait de réouvrir le fichier «
    readme.txt » fourni avec le logiciel.
-   Correction du problème lié à la fonction « Rouvrir le dernier
    document fermé » qui tentait de rouvrir le fichier README fourni
    avec le logiciel.
-   Correction de la barre de titre qui ne s\'actualisait pas après la
    fermeture d\'un document à partir de la boîte de dialogue « Tous les
    documents ».
-   Correction d'un problème empêchant le redimensionnement de la boîte
    de dialogue WebView et provoquant son affichage avec une taille
    initiale très réduite.
-   Correction d\'un problème d\'affichage des titres dans les documents
    Word contenant des noms de styles spécifiques à une locale.
-   Correction d'un problème où l'onglet sélectionné n'était pas
    correctement mis en avant après le redémarrage de Paperback.
-   Si une sélection est active lorsque vous ouvrez la boîte de dialogue
    de comptage de mots, le nombre de mots que vous avez sélectionnés
    s'affiche désormais.
-   Les images devraient désormais s'afficher correctement dans la vue
    Web intégrée.
-   Amélioration de la gestion par Paperback des fichiers sur les
    lecteurs réseau Windows : lorsque vous cliquez sur « Afficher le
    fichier dans le dossier », le focus se place désormais correctement
    sur le fichier situé sur le stockage réseau, et les chemins d'accès
    ne contiennent plus de caractères étranges.
-   Analyse syntaxique AZW3 considérablement améliorée.
-   Nous sommes passés de chmlib à notre propre lecteur de fichiers CHM
    entièrement écrit en Rust.
-   Sur ordinateur, les fichiers .paperback ne seront plus chargés de
    force lors de la restauration d'un document. À la place, une
    confirmation vous sera demandée lorsque le fichier sera détecté.
-   Paperback recourt désormais à l'extraction de texte brut pour les
    PDF mal étiquetés .
-   L'option « Ouvrir le dossier contenant » met désormais en
    surbrillance le fichier en question dans l'Explorateur.
-   L\'ouverture du fichier « readme » respectera désormais la langue
    que vous avez sélectionnée.
-   Les documents PowerPoint prennent désormais en charge les tableaux.
-   Mise à jour correcte du menu et mise en surbrillance du champ de
    texte lors de l' ouverture de l'aide dans Paperback.
-   Le fichier « Readme.html » ne sera plus ajouté à votre liste « Tous
    les documents » lorsqu'il est ouvert via Maj+F1.
-   La suppression de documents de la boîte de dialogue « Récents »
    ferme désormais également leur onglet actif.
-   Passage à une méthode d'IPC beaucoup plus sécurisée sous Windows.
-   Le titre du document actif est désormais lu lors du passage d'un
    onglet à l'autre.
-   Le programme de mise à jour affiche désormais correctement le
    contenu des balises de code Markdown dans les notes de mise à jour.
-   Le programme de mise à jour vérifie désormais que le fichier
    téléchargé n'a pas été altéré .
-   La vue Web s\'ouvre désormais à votre position de lecture actuelle.
-   Votre filtre de recherche dans la boîte de dialogue « Tous les
    documents » est désormais conservé après la suppression d\'un
    document.

### Version 0.8.5

-   Ajout de la prise en charge des pages pour les livres au format
    ePub.
-   Ajout de la prise en charge des documents Microsoft Office chiffrés.
    Actuellement, les versions héritées de Word, la version moderne de
    Word et la version moderne de PowerPoint sont prises en charge ; la
    prise en charge de la version héritée de PowerPoint est prévue pour
    l'avenir.
-   Ajout de la prise en charge des documents Microsoft Word hérités
    (\*.doc) !
-   Ajout de la prise en charge des présentations PowerPoint héritées
    (\*.ppt) !
-   Ajout de la prise en charge des livres au format mobi et AZW3 !
-   Ajout de la prise en charge des fichiers PDF balisés !
-   Ajout du raccourci Ctrl+Q pour quitter l\'application.
-   Ajout de la prise en charge des livres compressés provenant de
    Bookshare (au format DAISY et Word) !
-   Le texte alternatif des images intégrées devrait désormais
    s\'afficher correctement.
-   Les documents CHM prennent désormais correctement en charge la
    navigation via les liens internes.
-   Correction du problème : les sons des signets se déclenchaient au
    début du paragraphe au lieu de la position du signet.
-   Correction du décalage de 1 page lors de l\'accès à une page
    spécifique.
-   Correction du dysfonctionnement de la touche Échap pour fermer la
    boîte de dialogue « Ouvrir en tant que ».
-   Correction d\'un problème empêchant le menu contextuel du lecteur de
    s\'afficher lors d\'un clic droit ou en appuyant sur la touche
    Applications.
-   Correction d'un problème où le mauvais document était parfois
    sélectionné lors de l'ouverture de documents depuis la ligne de
    commande.
-   Les PDF contenant uniquement des images sont à nouveau détectés et
    leur présence vous est signalée.
-   Il est désormais possible de naviguer entre les images et les
    figures à l'aide des raccourcis g/Maj+g et f/Maj+f, respectivement.
-   Paperback respectera désormais le paramètre de mode sombre de votre
    application.
-   Suppression de la prise en charge du format DAISY XML, celle-ci
    n'étant plus nécessaire.
-   Retour à la navigation native Win32 par première lettre dans l'
    arborescence de la table des matières.
-   La boîte de dialogue d\'erreur de chargement affiche désormais des
    messages d\'erreur plus détaillés.
-   La vue Web s\'ouvre désormais beaucoup plus rapidement et de manière
    plus fluide.

### Version 0.8.2

-   Ajout de la prise en charge des pages dans les documents RTF !
-   Correction d'un bug qui provoquait l'activation automatique des
    liens externes lors de l'ouverture de la vue Web dans des fichiers
    ePub.
-   Correction d\'un bug qui, dans de rares cas, empêchait l\'analyseur
    RTF d\'insérer un espace entre les mots .
-   Correction d'un problème où les paragraphes étaient divisés en
    plusieurs courtes lignes dans certains documents PDF.
-   Les documents PDF prennent désormais en charge la navigation de base
    via les liens et les titres !
-   Les tabulations et les sauts de ligne RTF sont désormais affichés
    exactement comme ils apparaissent dans le document.
-   Retour à la bibliothèque pdfium, qui a fait ses preuves, pour
    l'analyse des PDF, ce qui rend à nouveau le rendu des PDF beaucoup
    plus fiable.

### Version 0.8.1

-   Ajout de la combinaison Ctrl+Maj+T pour rouvrir le dernier document
    fermé.
-   La boîte de dialogue « Tous les documents » permet désormais de
    sélectionner plusieurs documents à ouvrir simultanément.
-   Correction de quelques bogues liés à l\'analyseur RTF.
-   Correction d'un problème où les chemins d'accès contenant des
    caractères non ASCII (tels que les caractères bosniaques š, č, ć, ž)
    étaient corrompus lors de l'ouverture d'un fichier via une deuxième
    instance de Paperback .
-   Correction de la lecture du texte PDF dans le mauvais ordre et de
    l\'espacement incorrect autour des mots en majuscules.
-   Correction du chargement lent des documents lors de l\'ouverture de
    fichiers volumineux.
-   Correction de la localisation des boutons « Oui »/« Non » dans les
    boîtes de dialogue de confirmation. Version 0.8.0

### Version 0.8.0

-   Ajout des traductions en japonais, chinois simplifié et vietnamien !
-   Ajout d'un programme de mise à jour automatique qui remplacera
    désormais votre version actuelle de Paperback au lieu de simplement
    télécharger la nouvelle version !
-   Ajout d'un retour sonore optionnel lorsque vous atteignez un signet
    ou une note, merci à André Louis pour les sons !
-   Prise en charge des documents RTF ajoutée !
-   Ajout de la prise en charge des documents DAISY XML.
-   Ajout de la prise en charge des fichiers texte Flat Open Document !
-   Ajout de la prise en charge des présentations Flat Open Document !
-   Ajout de la prise en charge des séparateurs avec les touches « s »
    et « Maj + s ».
-   Tout déplacement de plus de 300 caractères sera désormais
    automatiquement ajouté à votre historique de navigation.
-   Correction de la restauration de la fenêtre de Paperback à partir de
    la barre d'état système.
-   Correction d\'un problème où les documents Markdown s\'affichaient
    en texte brut au lieu du code HTML rendu dans la vue Web.
-   Correction de l\'affichage incorrect des tableaux dans les fichiers
    Markdown.
-   Les PDF contenant uniquement des images vous avertiront désormais de
    leur présence lorsque vous tenterez d'en charger un.
-   Il est désormais possible de rechercher de nouvelles versions de
    développement plutôt que des versions stables lors de la
    vérification des mises à jour.
-   Intégration correcte des informations de version dans l\'exécutable
    de Paperback.
-   La boîte de dialogue des options a été divisée en onglets pour
    faciliter l'utilisation et la navigation.
-   Passage à Hayro pour l\'analyse des PDF, ce qui se traduit par une
    plus grande fiabilité, une vitesse accrue et un nombre réduit de
    DLL.
-   Réécriture complète de l'application en Rust. La nouvelle base de
    code est plus sûre, charge les documents plus rapidement et est plus
    facile à maintenir et à étendre.
-   Le menu contextuel du contrôle de texte inclut désormais des actions
    spécifiques au lecteur au lieu d'éléments génériques tels que «
    couper » et « coller ».

### Version 0.7.0

-   Ajout de la prise en charge des tableaux pour les documents HTML et
    XHTML ! Naviguez entre les tableaux à l'aide des touches T et Maj+T,
    et appuyez sur Entrée pour en afficher un dans une vue Web.
-   Ajout d'une fonctionnalité de rendu Web de base ! Appuyez sur
    Ctrl+Maj+V pour ouvrir la section actuelle de votre document dans un
    rendu Web, ce qui est utile pour les contenus présentant une mise en
    forme complexe ou des exemples de code.
-   Ajout d'une traduction en russe, merci à Ruslan Gulmagomedov !
-   Ajout d\'un bouton « Tout effacer » dans la boîte de dialogue « Tous
    les documents ».
-   Le vérificateur de mises à jour affiche désormais les notes de mise
    à jour lorsqu'une nouvelle version est disponible.
-   Correction de la restauration de la fenêtre à partir de la barre
    d\'état système.
-   Correction des traductions des boutons « Oui »/« Non » dans les
    boîtes de dialogue de confirmation.
-   Correction du chargement des configurations lors de l\'exécution en
    tant qu\'administrateur.
-   Correction de la gestion des commentaires dans les documents XML et
    HTML.
-   Correction de l\'analyse de la table des matières dans les livres au
    format Epub 2.
-   Correction de la navigation vers l\'élément suivant portant la même
    lettre dans la table des matières.
-   Correction du problème empêchant la boîte de dialogue de recherche
    de se masquer correctement lors de l\'utilisation des boutons «
    Suivant » et « Précédent ».
-   Correction d\'un problème où les tables des matières ePub
    redirigeaient parfois vers le mauvais élément.
-   Correction de divers problèmes de gestion des espaces dans les
    balises XML, HTML et .
-   Correction d\'une erreur « off-by-one » dans la navigation par
    liens.
-   Correction de l\'apparition d\'espaces finaux à la fin des lignes
    dans certains livres.
-   Correction de divers problèmes liés à l\'analyse syntaxique.
-   Les éléments de menu liés aux signets ainsi que la liste des
    éléments sont désormais correctement désactivés lorsqu\'aucun
    document n\'est ouvert.
-   Amélioration de la gestion des listes dans divers formats de
    documents.
-   Amélioration du flux de travail de traduction pour les
    contributeurs.
-   Nombreuses refactorisations internes : migration de la majeure
    partie de la logique métier de l'application du C++ vers Rust pour
    améliorer les performances et la facilité de maintenance.

### Version 0.6.1

-   Ajout de la prise en charge des PDF protégés par mot de passe !
-   Ajout d'une fonctionnalité très basique permettant d'accéder à la
    position précédente/suivante. Si vous appuyez sur Entrée sur un lien
    interne et que cela déplace votre curseur, cette position sera
    désormais mémorisée, et vous pourrez y accéder à l'aide des touches
    Alt + flèches gauche/droite .
-   Ajout d'une liste d'éléments ! Actuellement, elle affiche uniquement
    une arborescence de tous les titres de votre document ou une liste
    de liens, mais il est prévu de l'étendre à l'avenir.
-   Ajout d'une option permettant de lancer Paperback en mode maximisé
    par défaut.
-   Correction d\'un problème empêchant le bon fonctionnement des liens
    dans certains documents ePub.
-   Correction de l'analyse des tables des matières ePub contenant des
    chemins relatifs.
-   Correction d'un problème empêchant certains documents ePub
    d'afficher le titre ou l'auteur.
-   Correction d\'un problème où les titres de certains chapitres ePub
    ne s\'affichaient pas correctement dans la boîte de dialogue de la
    table des matières.
-   Correction d\'un problème empêchant l\'utilisation de la barre
    d\'espace pour activer les boutons « OK »/« Annuler » dans la boîte
    de dialogue de la table des matières.
-   Amélioration de la gestion des titres dans les documents Word.
-   Vous recevrez désormais un message vocal si la liste des documents
    récents est vide lorsque vous essayez d\'afficher la boîte de
    dialogue.

### Version 0.6.0

-   Une nouvelle option permettant d'afficher le menu « Aller à » sous
    une forme bien plus compacte a été ajoutée à la boîte de dialogue
    des options ; elle est cochée par défaut.
-   Ajout d'une option permettant de faire défiler la navigation par
    éléments structurels.
-   Ajout d'une option dans le menu « Outils » permettant d'ouvrir le
    dossier contenant le document actuellement sélectionné.
-   Ajout d'un système de mise à jour assez simple, mais très efficace.
-   Ajout d'une fonctionnalité de mise en veille de base, accessible via
    Ctrl+Maj+S.
-   Ajout de la prise en charge de l\'analyse des livres électroniques
    au format FB2 !
-   Ajout de la prise en charge de l\'analyse des présentations
    OpenDocument !
-   Ajout de la prise en charge de l\'analyse des fichiers OpenDocument
    Text !
-   Les signets peuvent désormais être créés pour marquer une ligne
    entière ou pour ne marquer que du texte spécifié. Si aucune
    sélection n\'est active lorsque vous placez un signet, le
    comportement est identique à celui des versions antérieures à la
    0.6, et la ligne entière sera marquée. Cependant, si vous
    sélectionnez du texte, seul ce texte sera inclus dans le signet.
-   Les signets peuvent désormais être accompagnés de notes textuelles
    facultatives ! Naviguez entre les signets contenant des notes à
    l'aide des touches N et Maj+N, ou affichez la boîte de dialogue des
    signets avec tous les signets, uniquement les notes ou uniquement
    les signets sans note sélectionnés à l'aide de raccourcis clavier
    spécifiques.
-   Les signets de la boîte de dialogue des signets n'auront plus le
    préfixe gênant « signet x ».
-   Les livres au format Epub contenant du contenu HTML se présentant
    comme du XML seront désormais gérés correctement.
-   Correction du chargement des documents Markdown volumineux.
-   Correction du problème où appuyer sur la barre d'espace dans
    l'arborescence de la table des matières activait le bouton OK.
-   Correction de la gestion des espaces au début des balises \`pre\`
    dans les documents HTML et XHTML.
-   Correction d'un problème où le champ de texte ne récupérait parfois
    pas le focus lors du retour à la fenêtre de Paperback.
-   Correction d'un problème où le champ de texte de la boîte de
    dialogue « Aller à % » ne mettait pas à jour la valeur du curseur.
-   Correction du rendu des identifiants HTML personnalisés dans les
    documents Markdown.
-   Le code HTML contenu dans les blocs de code Markdown s'affiche
    désormais correctement.
-   Si vous chargez un livre à l'aide d'un paramètre de ligne de
    commande alors qu'une instance existante de Paperback est en cours
    d'exécution, vous n'obtiendrez plus d'erreur si le chargement de
    votre document prend plus de 5 secondes.
-   Si vous exécutez Paperback en tant qu'administrateur, la
    configuration sera désormais correctement chargée et enregistrée.
-   Il est désormais possible de supprimer un signet directement depuis
    la boîte de dialogue des signets.
-   Il est désormais possible d'importer et d'exporter vos signets ainsi
    que votre position de lecture pour un document donné. Le fichier
    généré porte le même nom que le document, avec l'extension
    .paperback. Si un tel fichier se trouve dans le même répertoire
    qu'un document lors de son chargement, il sera automatiquement
    chargé. Sinon, vous pouvez les importer manuellement à l'aide d'une
    option du menu Outils.
-   Les liens à l'intérieur des documents sont désormais entièrement
    pris en charge ! Utilisez k et Maj+k pour parcourir ces liens vers
    l'avant et vers l'arrière, puis appuyez sur Entrée pour
    ouvrir/activer l'un d'entre eux.
-   De nombreuses refactorisations internes ont été effectuées, rendant
    l'application plus rapide et le fichier binaire plus léger.
-   Le contenu Markdown est désormais prétraité afin d'être conforme à
    la norme CommonMark avant le rendu.
-   La navigation par listes et leurs éléments est désormais entièrement
    prise en charge ! Utilisez L et Maj+L pour parcourir les listes
    elles-mêmes, et I et Maj+I pour parcourir les éléments des listes.
-   La touche Suppr du pavé numérique permet désormais de supprimer des
    documents de la barre d'onglets, en plus de la touche Suppr
    standard.
-   Paperback peut désormais, si vous le souhaitez, se réduire dans
    votre barre d'état système ! Cette option est désactivée par défaut,
    mais en l'activant, l'option de réduction du menu système placera
    Paperback dans votre barre d'état système, d'où vous pourrez le
    restaurer en cliquant sur l'icône qui s'y trouve.
-   Paperback est désormais entièrement traduisible ! La liste des
    langues qu'il prend en charge est pour l'instant assez restreinte,
    mais elle ne cesse de s'allonger !
-   Paperback dispose désormais d'un site web officiel, à l'adresse
    [paperback.dev](https://paperback.dev) !
-   Les documents PPTX affichent désormais une table des matières
    basique, contenant toutes les diapositives.
-   Le chemin d'accès complet au document ouvert s'affiche désormais
    dans la boîte de dialogue d'informations sur le document.
-   Le programme d'installation inclut désormais une option permettant
    de consulter le fichier Lisez-moi dans votre navigateur après
    l'installation.
-   La liste des documents récents a été considérablement élargie ! Au
    lieu de se contenter d'afficher les 10 derniers documents que vous
    avez ouverts, elle vous en présente désormais un nombre
    personnalisable, les autres documents que vous avez ouverts à un
    moment ou à un autre étant accessibles via une petite boîte de
    dialogue.
-   Diverses petites améliorations ont été apportées aux analyseurs
    syntaxiques, notamment l'insertion d'une ligne vide entre les
    diapositives dans les présentations PPTX, la correction de la
    gestion des sauts de ligne à l'intérieur des paragraphes dans les
    documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0

-   Prise en charge des documents Microsoft Word ajoutée !
-   Ajout de la prise en charge des présentations PowerPoint !
-   Correction d'un problème où certains éléments de menu n'étaient pas
    désactivés lorsqu'aucun document n'était ouvert.
-   Correction de l\'orientation du curseur de pourcentage « Aller à ».
-   Correction de la table des matières dans les livres au format ePub
    contenant des chemins d'accès aux fichiers encodés en URL et/ou des
    identifiants de fragment.
-   Correction d\'un problème de suppression des espaces dans les
    en-têtes XHTML de manière inattendue.
-   Correction de la gestion des espaces à l\'intérieur des balises
    \`pre\` imbriquées dans les documents HTML.
-   Les documents HTML et Markdown prennent désormais en charge la
    fonctionnalité de table des matières ! Lorsque vous chargez un
    document HTML/Markdown, Paperback génère sa propre table des
    matières à partir de la structure des titres de votre document, et
    vous la présente dans la boîte de dialogue accessible via Ctrl+T.
-   Les documents HTML auront désormais pour titre celui défini dans la
    balise \`title\`, s'il existe. Sinon, ils continueront d'utiliser le
    nom de fichier sans l' extension.
-   Passage d'UniversalSpeech à l'utilisation d'une zone active pour la
    synthèse vocale. Cela signifie qu'aucune DLL de lecteur d'écran
    n'est désormais fournie avec le programme, et que davantage de
    lecteurs d'écran seront désormais pris en charge, tels que Microsoft
    Narrator.
-   Changement de bibliothèques ZIP pour permettre l'ouverture d'un plus
    large éventail de livres EPUB.
-   La boîte de dialogue vous demandant si vous souhaitez ouvrir votre
    document au format texte brut a été entièrement refaite ; elle vous
    permet désormais d'ouvrir votre document au format texte brut, HTML
    ou Markdown.
-   La boîte de dialogue « Aller au pourcentage » comprend désormais un
    champ de texte vous permettant de saisir manuellement un pourcentage
    vers lequel vous souhaitez vous rendre.
-   L\'analyseur HTML reconnaît désormais les balises dd, dt et dl comme
    des éléments de liste.
-   La table des matières des livres ePub sera à nouveau conservée à
    l'identique.
-   L\'espace insécable Unicode est désormais pris en compte lors de la
    suppression des lignes vides.
-   On ne vous demandera plus comment vous souhaitez ouvrir un fichier
    non reconnu à chaque fois que vous le chargez, mais uniquement la
    première fois.

### Version 0.4.1

-   Ajout d'une icône facultative dans le menu Démarrer via le programme
    d'installation.
-   La table des matières devrait désormais être plus claire dans
    certains cas ; par exemple, si vous avez un élément enfant et un
    élément parent avec le même texte à la même position, vous ne verrez
    désormais que l\'élément parent.
-   Correction de la table des matières dans certains documents CHM.
-   Correction de la table des matières dans les livres Epub 3 contenant
    des chemins d'accès absolus. Les
-   Les documents CHM devraient désormais afficher leur titre tel qu'il
    est défini dans le fichier de métadonnées .

### Version 0.4.0

-   Ajout de la prise en charge des fichiers CHM !
-   Prise en charge des signets ajoutée ! Vous pouvez créer autant de
    signets que vous le souhaitez dans autant de documents que vous le
    souhaitez. Vous pouvez vous déplacer vers l'avant et vers l'arrière
    parmi ces signets à l'aide des touches b et Maj+b, en créer un avec
    Ctrl+Maj+b, et afficher une boîte de dialogue pour accéder à un
    signet spécifique avec Ctrl+b.
-   Ajout d'un programme d'installation en plus du fichier ZIP portable
    ! Le programme d'installation installera Paperback dans votre
    répertoire « Program Files » et configurera automatiquement les
    associations de fichiers pour vous.
-   Les fichiers texte comportant une table de caractères (BOM)
    devraient désormais être décodés correctement, et la BOM
    n'apparaîtra plus au début du texte.
-   Ajout d'informations bien plus complètes dans la barre d'état. Elle
    vous indiquera désormais la ligne, le caractère et le pourcentage de
    lecture en cours.
-   Les commentaires HTML, ainsi que le contenu des balises de script et
    de style, n'apparaîtront plus dans la sortie texte.
-   Si vous passez un chemin relatif à Paperback en ligne de commande,
    celui-ci le résoudra désormais correctement.
-   Le déplacement en pourcentage est désormais géré par sa propre boîte
    de dialogue à curseur, accessible via Ctrl+Maj+G.
-   Les documents dont le titre ou l'auteur est inconnu auront désormais
    toujours une valeur par défaut.
-   La logique d'enregistrement de la position est désormais beaucoup
    plus intelligente et ne devrait écrire sur le disque qu'en cas
    d'absolue nécessité.
-   Le document sur lequel se trouvait le focus lorsque vous avez fermé
    Paperback est désormais mémorisé d'un redémarrage à l'autre.
-   Les entrées dans les boîtes de dialogue « Aller à la ligne » et «
    Aller à la page » devraient désormais être valider de manière plus
    stricte.
-   Correction de la navigation dans la table des matières des livres
    ePub 3 dont les manifestes contiennent des chemins relatifs.

### Version 0.3.0

-   Correction de la table des matières dans les livres ePub dont les
    manifestes sont encodés en URL. Correction de la navigation entre
    les titres dans les documents HTML contenant des caractères Unicode
    multi-octets.
-   Correction de la navigation entre les titres dans les documents HTML
    contenant des caractères Unicode multi-octets.
-   Correction d'une utilisation élevée du processeur dans les documents
    comportant de longs titres, due à une régression dans wxWidgets.
-   Correction du chargement des fichiers texte UTF-8.
-   Correction d\'un problème où les éléments imbriqués de la table des
    matières dans les livres ePub plaçaient le curseur à une mauvaise
    position.
-   Correction d\'un plantage à la fermeture de l\'application dans
    certains cas.
-   Ajout d'une case à cocher dans la boîte de dialogue des options pour
    activer ou désactiver le retour à la ligne automatique !
-   Il est désormais possible de faire un don pour soutenir le
    développement de Paperback, soit via la nouvelle option « Faire un
    don » du menu Aide, soit via le lien « Parrainer ce projet » situé
    au bas de la page principale du dépôt GitHub.
-   Les documents Markdown auront désormais toujours un titre, et
    Paperback devrait désormais pouvoir charger pratiquement n'importe
    quel fichier Markdown.
-   Les documents PDF auront désormais toujours un titre, même si les
    métadonnées sont absentes.
-   Passage à la bibliothèque PDF utilisée dans Chromium, ce qui se
    traduit par une analyse des PDF bien plus fiable dans l'ensemble.
-   Vous ne pouvez désormais exécuter qu'une seule instance de Paperback
    à la fois. Lancer paperback.exe avec un nom de fichier alors que le
    programme est déjà en cours d'exécution ouvrira ce document dans
    l'instance déjà en cours d'exécution.
-   Vous pouvez désormais appuyer sur la touche Suppr sur un document
    dans la barre d'onglets pour le fermer. Version 0.2.1

### Version 0.2.1

-   Ajout du nombre total de pages à l'étiquette de page dans la boîte
    de dialogue « Aller à la page ». Ajout de la possibilité de passer
    du contenu du document à votre liste de
-   Possibilité de passer du contenu du document à la liste des
    documents ouverts à l'aide de la touche Tab.
-   Correction d'un problème où les raccourcis clavier des en-têtes
    ouvraient parfois des documents récents si vous en aviez
    suffisamment.
-   Paperback supprime désormais les traits d\'union logiciels inutiles
    de la sortie de texte.
-   Correction d'un problème où la navigation par en-tête vous plaçait
    parfois sur le mauvais caractère.

### Version 0.2.0

-   Ajout de la prise en charge des documents Markdown !
-   Ajout de la prise en charge des documents PDF, y compris la
    possibilité de naviguer entre les pages !
-   Ajout de raccourcis clavier pour naviguer par titres dans le contenu
    HTML, y compris les livres au format ePub et les documents Markdown.
    Ces raccourcis ont été conçus pour fonctionner de manière similaire
    à un lecteur d\'écran.
-   Correction du chargement des livres ePub dont les noms de fichiers
    sont encodés en URL dans leurs manifestes.
-   Correction du chargement des livres ePub 3 contenant du code XHTML
    intégré.
-   Un message est désormais lu à voix haute si le document ne prend pas
    en charge de table des matières ou de sections, au lieu de
    désactiver les éléments de menu.
-   Ajout d'un menu « Documents récents » ! Il stocke actuellement vos
    10 derniers documents ouverts, et appuyer sur Entrée sur l'un
    d'entre eux l'ouvrira pour la lecture.
-   La boîte de dialogue « Rechercher » a été entièrement réécrite, ce
    qui la rend beaucoup plus simple à utiliser, tout en ajoutant un
    historique de vos 25 dernières recherches et la prise en charge des
    expressions régulières !
-   Les documents précédemment ouverts sont désormais mémorisés même
    après un redémarrage de l'application. Cette fonctionnalité est
    configurable via le nouvel élément d'options du menu Outils.
-   Ajout de la combinaison Maj+F1 pour ouvrir le fichier « Lisez-moi »
    directement dans Paperback.

### Version 0.1.0

-   Première version.
