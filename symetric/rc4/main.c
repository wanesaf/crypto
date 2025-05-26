#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "bib.h"

unsigned char message_chiffre[MAX];
int main() {
    printf("Entrez la cle: ");
    fgets(cle, MAX, stdin);
    cle[strcspn(cle, "\n")] = '\0'; 

    printf("Entrez le message : ");
    fgets(message, MAX, stdin);
    message[strcspn(message, "\n")] = '\0'; 

    initialiser(cle);//initialiser la permutation

    
    flux(message, message_chiffre, strlen(message)); // on genere le flux 

    for (int i = 0; i < strlen(message_chiffre); i++) {
        // l'affichage est en hexa , si tu veux l'afficher en utf-8 faire %s mais yakhrodj mindak les lettres mhabel
        printf("%02X ", (unsigned char)message_chiffre[i]);
    }
    return 0;
}