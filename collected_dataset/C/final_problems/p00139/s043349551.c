#include <stdio.h>
#define TRUE 1
#define FALSE 0

int main(void)
{
    char A, B;
    char snake[256];
    int n;
    int cnt;
    int i, j, len1, len2;
    
    scanf("%d", &n);
    
    for (i = 0; i < n; i++){
        A = B = FALSE;
        scanf("%s", snake);
        
        if (snake[0] == '>' && snake[1] == '\''){
            j = 2;
            len1 = 0;
            while (snake[j] != '#' && snake[j] != '\0'){
                len1++;
                j++;
            }
            if (snake[j] == '#'){
                j++;
                len2 = 0;
                while (snake[j] != '~' && snake[j] != '\0'){
                    len2++;
                    j++;
                }
                if (len1 == len2 && len1 > 0 && snake[j + 1] == '\0'){
                    A = TRUE;
                }
            }
        }
        else if (snake[0] == '>' && snake[1] == '^'){
            j = 2;
            len1 = 0;
            len2 = 1;
            while (snake[j] != '~'){
                if (j % 2 == 0 && snake[j] != 'Q'){
                    len2 = 0;
                    break;
                }
                if (j % 2 == 1 && snake[j] != '='){
                    len2 = 0;
                    break;
                }
                j++;
                len1++;
            }
            
            if (len2 && snake[j + 1] == '~' && len1 > 0 && len1 % 2 == 0 && snake[j + 2] == '\0'){
                B = TRUE;
            }
        }
        
        if (A){
            printf("A\n");
        }
        else if (B){
            printf("B\n");
        }
        else {
            printf("NA\n");
        }
    }
    
    return (0);
}