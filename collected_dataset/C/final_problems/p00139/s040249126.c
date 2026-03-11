#include <stdio.h>
#include <string.h>

int check_a(char *snake)
{
    int n;
    
    if (snake[strlen(snake) - 1] != '~'){
        return (0);
    }
    snake[strlen(snake) - 1] = '\0';

    if (*snake != '='){
        return (0);
    }
    
    n = 0;
    while (*snake != '#'){
        if (*snake != '='){
            return (0);
        }
        n++;
        snake++;
    }
    snake++;
    while (n > 0){
        if (*snake != '='){
            return (0);
        }
        n--;
        snake++;
    }
    if (*snake != '\0'){
        return (0);
    }
    return (1);
}

int check_b(char *snake)
{
    int n;
    
    if (snake[strlen(snake) - 1] != '~' || 
        snake[strlen(snake) - 2] != '~'){
        return (0);
    }
    snake[strlen(snake) - 2] = '\0';

    if (*snake != 'Q'){
        return (0);
    }
    
    while (*snake != '\0'){
        if (*snake != 'Q'){
            return (0);
        }
        snake++;
        if (*snake != '='){
            return (0);
        }
        snake++;
    }
    return (1);
}

int main(void)
{
    int n;
    char snake[1000];
    int i;
    
    scanf("%d", &n);
    for (i = 0; i < n; i++){
        scanf("%s", snake);
        if (snake[0] == '>'){
            if (snake[1] == '\''){
                if (check_a(&snake[2]) == 1){
                    printf("A\n");
                    continue;
                }
            }
            else if (snake[1] == '^'){
                if (check_b(&snake[2]) == 1){
                    printf("B\n");
                    continue;
                }
            }
        }
        printf("NA\n");
    }
    return (0);
}