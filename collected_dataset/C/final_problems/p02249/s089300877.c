#include<stdio.h>
int a[1000][1000], b[1000][1000];
int check(int x, int y, int r, int c){
    int i, j;
    for(i=0; i<r; i++)for(j=1; j<c+1; j++)if(a[i+x][j+y]!=b[i][j])return 0;
    return 1;
}
int main(){
    int i, j, h, w, r, c;
    scanf("%d %d", &h, &w);
    for(i=0; i<h; i++)for(j=0; j<w+1; j++)a[i][j]=getchar();
    scanf("%d %d", &r, &c);
    for(i=0; i<r; i++)for(j=0; j<c+1; j++)b[i][j]=getchar();
    for(i=0; i<h-r+1; i++)for(j=0; j<w-c+1; j++)if(check(i, j, r, c))printf("%d %d\n", i, j);
    return 0;
}