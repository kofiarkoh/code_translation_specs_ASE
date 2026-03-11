#include <stdio.h>

int main(void){
    int h, w;
    char a[100+5][100+5];
    scanf("%d %d", &h, &w);
    for(int i=0; i<h; i++)scanf("%s", a[i]);
    // check bit
    for(int i=0; i<h; i++)a[i][w]='x';
    for(int i=0; i<w; i++)a[h][i]='x';
    
    for(int i=0; i<h; i++){
        for(int j=0; j<w; j++){
            if(a[i][j]=='#'){
                a[i][w]='o';
                a[h][j]='o';
            }
        }
    }
    
    for(int i=0; i<h; i++){
        if(a[i][w]=='o'){
            for(int j=0; j<w; j++){
                if(a[h][j]=='o')printf("%c", a[i][j]);
            }
            printf("\n");
        }
    }
    return 0;
}
