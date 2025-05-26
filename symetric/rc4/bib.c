#include "bib.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int permutation[MAX];
unsigned char cle[MAX],message[MAX]; 

void permuter(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

void initialiser(const unsigned char *cle) {
    for (int i = 0; i < MAX; i++)
        permutation[i] = i;

    int j = 0;
    for (int i = 0; i < MAX; i++) {
        j = (j + permutation[i] + cle[i % strlen(cle)]) % MAX;
        permuter(&permutation[i], &permutation[j]);
    }
}

void flux(const unsigned char *msg, unsigned char *resultat, int taille_msg) {
    int i = 0, j = 0;
    for (int k = 0; k < taille_msg; k++) {
        i = (i + 1) % MAX;
        j = (j + permutation[i]) % MAX;
        permuter(&permutation[i], &permutation[j]);
        int octet_chiffrement = permutation[(permutation[i] + permutation[j]) % MAX];
        resultat[k] = msg[k] ^ octet_chiffrement; 
}
}