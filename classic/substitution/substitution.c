#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#define MAXSIZE 26
#define MAX_CHAR 100
int size = 0;

char keys[MAXSIZE];  
char values[MAXSIZE]; 

int getIndex(char key) {
    for (size_t i = 0; i < size; i++) {
        if (keys[i] == key) {  
            return i;
        }
    }
    return -1;
}

void insert(char key, char value) {
    int index = getIndex(key);
    if (index == -1) {
        if (size < MAXSIZE) { 
            keys[size] = key;
            values[size] = value;
            size++;
        }
    } else {
        values[index] = value;
    }
}

char getValue(char key) {
    int index = getIndex(key);
    if (index == -1) {
        return key; 
    }
    return values[index];
}

void printMap() {
    for (size_t i = 0; i < size; i++) {
        printf("%c : %c\n", keys[i], values[i]);
    }
}

void fillMap() {
    //Our map 
    insert('A','N');
    insert('B','A');
    insert('C','H');
    insert('D','Y');
    insert('E','X');
    insert('F','P');
    insert('G','O');
    insert('H','E');
    insert('I','K');
    insert('J','J');
    insert('K','D');
    insert('L','I');
    insert('M','U');
    insert('N','G');
    insert('O','Q');
    insert('P','Z');
    insert('Q','W');
    insert('R','B');
    insert('S','T');
    insert('T','L');
    insert('U','S');
    insert('V','F');
    insert('W','R');
    insert('X','C');
    insert('Y','V');
    insert('Z','M');
}

int main() {
    
    fillMap();

    char word[MAX_CHAR];

    printf("Enter the word to encrypt: ");
    fflush(stdout);
    fgets(word,MAX_CHAR,stdin);

    
   
    size_t len = strlen(word);

    if (word[len-1]=='\n') {
        word[len-1] = '\0';
        len--;
    }

    for (size_t i = 0 ; i < len ; i++) {
        word[i] = toupper(word[i]); // we transform lowers to upper so the encryption is the same if we have upperword
    }


    char *encrypted_word = (char*)malloc(len +1); 

     if (encrypted_word == NULL) {
        perror("Memory allocation failed");
        exit(EXIT_FAILURE);
     }

     for (size_t i = 0 ; i < len ; i++) {
        encrypted_word[i] = getValue(word[i]);
     }

     encrypted_word[len] = '\0';

     printf("The encrypted word is: %s\n", encrypted_word);

     free(encrypted_word);

    return 0;
}
