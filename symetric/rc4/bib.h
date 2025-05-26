#ifndef BIB_H_ 
#define BIB_H_

#define MAX 256
/// @brief 
// on utilise unsigned car on manipule des octets et un octet doit contenir des valuers de 0 a 255 donc unsigned est le meuilleur choix
extern int permutation[MAX];
extern unsigned char cle[MAX], message[MAX];

void permuter(int *a, int *b);
void initialiser(const unsigned  char *cle);
void flux(const unsigned char *msg, unsigned char  *resultat, int taille_msg);

#endif
