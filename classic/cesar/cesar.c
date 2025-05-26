#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_CHAR 100
#define MAX_KEY 4

int main()
{
    char word[MAX_CHAR];

    printf("Enter the word to encrypt: ");
    fflush(stdout);
    fgets(word, MAX_CHAR, stdin);

    size_t len = strlen(word);
    if (word[len - 1] == '\n')
    {
        word[len - 1] = '\0';
        len--;
    }

    printf("The word to encrypt is: %s\n", word);

    for (size_t i = 0; i < len; i++)
    {
        word[i] = toupper(word[i]);
    }

    char key[MAX_KEY];

    printf("Enter the encryption key: ");
    fflush(stdout);
    fgets(key, MAX_KEY, stdin);

    if (key[strlen(key) - 1] == '\n')
    {
        key[strlen(key) - 1] = '\0';
    }

    int k = atoi(key);

    printf("The key is: %d\n", k);

    if (k == 0)
    {
        printf("The key is 0, nothing changes: %s\n", word);
    }
    else
    {

        char *encrypted_word = (char *)malloc(len + 1);
        if (encrypted_word == NULL)
        {
            perror("Memory allocation failed");
            exit(EXIT_FAILURE);
        }

        for (size_t i = 0; i < len; i++)
        {
            if ('A' <= word[i] &&  word[i] <= 'Z')
            {
                encrypted_word[i] = ((int)word[i] - 'A' +k) % 26 + 'A'; 
            }
            else {
                encrypted_word[i] = word[i];//nkhaliwha kima rahi zaema space wela another non latin character
            }
        }

        encrypted_word[len] = '\0';

        printf("The encrypted word is: %s\n", encrypted_word);

        free(encrypted_word);
    }

    return 0;
}
