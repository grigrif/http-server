Ce projet est un serveur HTTP simple, écrit en Rust pour comprendre le protocol http 

Voici les principales routes configurées dans le serveur :

GET / : Affiche une page html
GET /user-agent : Renvoie le User-Agent des headers 
GET /say/* : Repete la suite de l'url en réponse 
GET /files/* : Permet de récupérer des fichiers où * représente le nom du fichier demandé (répertoire test par défaut sinon on peut)
POST /file/* : Créer un fichier où * est le nom du fichier représente. Le body de la requete correspond au contenu du fichier.

Le serveur gère également la compression gzip si le client l'accepte  

