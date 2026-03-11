#include <stdio.h>

int main(void)
{
    int one, temp;
    int n;
    int i;
    char str[512];
    int score;
    
    while (fgets(str, 512, stdin) != NULL){
        one = score = temp = i = 0;
        while (str[i] != '\0'){
            temp = 0;
            while (str[i] != ' ' && str[i] != '\n'){
                temp = temp * 10 + str[i++] - '0';
            }
            if (temp == 0){
                return (0);
            }
            if (temp != 1){
                if (temp > 10){
                    temp = 10;
                }
                score += temp;
            }
            else {
                one++;
            }
            i++;
        }
        
        score += one;
        if (score > 21){
            printf("0\n");
        }
        else if (one == 0){
            printf("%d\n", score);
        }
        else {
            while (one-- && score + 10 <= 21){
                score += 10;
            }
            printf("%d\n", score);
        }
    }
    
    return (0);
}