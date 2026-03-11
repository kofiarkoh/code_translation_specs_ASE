#include <stdio.h>

void yoko(int w)
{
    printf("+");
    while (w >= 3){
        printf("-");
        w--;
    }
    printf("+\n");
}

void tate(int w)
{
    printf("|");
    while (w >= 3){
        printf(".");
        w--;
    }
    printf("|\n");
}

void center(int w, char *c)
{
    int i;
    
    printf("|");
    for (i = 0; i < (w - 3) / 2; i++){
        printf(".");
    }
    printf("%s", c);
    for (i = 0; i < (w - 3) / 2; i++){
        printf(".");
    }
    printf("|\n");
}

int main(void)
{
    int W, H;
    char c[2];
    int i;
    
    scanf("%d%d%s", &W, &H, c);
    
    yoko(W);
    for (i = 0; i < (H - 3) / 2; i++){
        tate(W);
    }
    center(W, c);
    for (i = 0; i < (H - 3) / 2; i++){
        tate(W);
    }
    yoko(W);
    
    return (0);
}

