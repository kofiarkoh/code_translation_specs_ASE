#include<stdio.h>

int main(){
    int i,j,k,n,m,x,y;
    char b[101][101];

    scanf("%d",&n);
    while(n--){
        for(i=0;i<101;i++){
            for(j=0;j<101;j++){
                b[i][j]=' ';
            }
        }
        scanf("%d",&m);

        x=m;y=1;
        b[x][y]='#';
        for(i=0,k=m-1;i<m;i++){
            for(j=0;j<k;j++){
                if(i%4==0){
                    //puts("0");
                    b[--x][y]='#';
                    //if(i!=0||j!=k-1)x--;
                }
                if(i%4==1){
                    //puts("1");
                    b[x][++y]='#';
                    //y++;
                }
                if(i%4==2){
                    //puts("2");
                    b[++x][y]='#';
                    //x++;
                }
                if(i%4==3){
                    //puts("3");
                    b[x][--y]='#';
                    //y--;
                }
                //printf("x:%d y:%d k:%d\n",x,y,k);
            }
            if(i==0)k;
            else if(i%2==0){
                k-=2;
            }
        }
        for(i=1;i<=m;i++){
            for(j=1;j<=m;j++){
                printf("%c",b[i][j]);
            }
            puts("");
        }if(n)puts("");
    }
    return 0;
}