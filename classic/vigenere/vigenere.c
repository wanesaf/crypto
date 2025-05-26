#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

#define MAXSIZE 26
#define MAX_CHAR 500

int main()
{
    char word[MAX_CHAR];
    char choice[3]; 

    printf("Menu\n");
    printf("1 - Encrypt\n");
    printf("2 - Decrypt\n");

    printf("\nEnter your choice: ");
    fflush(stdout);
    fgets(choice, sizeof(choice), stdin);

    
    size_t len_choice = strlen(choice);
    if (choice[len_choice - 1] == '\n')
    {
        choice[len_choice - 1] = '\0';
    }

    printf("\nEnter the word: ");
    fflush(stdout);
    fgets(word, MAX_CHAR, stdin);

    size_t len_word = strlen(word);
    if (word[len_word - 1] == '\n')
    {
        word[len_word - 1] = '\0';
        len_word--;
    }

    for (size_t i = 0; i < len_word; i++)
    {
        word[i] = toupper(word[i]);
    }

    char *key = (char *)malloc(MAX_CHAR);
    if (key == NULL)
    {
        perror("Memory allocation failed");
        exit(EXIT_FAILURE);
    }

    printf("\nEnter the key: ");
    fflush(stdout);
    fgets(key, MAX_CHAR, stdin);

    size_t len_key = strlen(key);
    if (key[len_key - 1] == '\n')
    {
        key[len_key - 1] = '\0';
        len_key--;
    }

    for (size_t j = 0; j < len_word; j++)
    {
        key[j] = toupper(key[j]);
    }

    

    char *new_word = (char *)malloc(len_word + 1);
    if (new_word == NULL)
    {
        perror("Memory allocation failed");
        free(key);
        exit(EXIT_FAILURE);
    }

    size_t j_key = 0;

    if (strcmp(choice, "1") == 0)  
    {
        for (size_t i = 0; i < len_word; i++)
        {
            if (j_key == len_key)
            {
                j_key = 0;
            }
            new_word[i] = ((word[i] - 'A') + (key[j_key] - 'A')) % 26 + 'A';
            j_key++;
        }
    }
    else if (strcmp(choice, "2") == 0)  
    {
        for (size_t i = 0; i < len_word; i++)
        {
            if (j_key == len_key)
            {
                j_key = 0;
            }
            new_word[i] = ((word[i] - 'A') - (key[j_key] - 'A') + 26) % 26 + 'A';
            j_key++;
        }
    }
    else
    {
        printf("Invalid choice. Exiting.\n");
        free(key);
        free(new_word);
        return 1;
    }

    new_word[len_word] = '\0';

    printf("\nThe result is: %s\n", new_word);

    free(key);
    free(new_word);

    return 0;
}
