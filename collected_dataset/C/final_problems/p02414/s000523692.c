#include <stdio.h>

int main(int argc, const char * argv[]) {
    int a,b,c;;
    long sum;
    int i,j,r;
    int x[100][100],y[100][100];
    scanf("%d %d %d",&a,&b,&c);
    for(i=0;i<a;i++){
        for(j=0;j<b;j++){
            scanf("%d",&x[i][j]);
        }
    }
    for(j=0;j<b;j++){
        for(r=0;r<c;r++){
            scanf("%d",&y[j][r]);
        }
    }
    for(i=0;i<a;i++){
        for(r=0;r<c;r++){
            sum = 0;
            for(j=0;j<b;j++){
                sum+=x[i][j]*y[j][r];
                if(j==b-1){
                    printf("%ld ",sum);
                }
            }
        }
        printf("\n");
    }
    return 0;
}