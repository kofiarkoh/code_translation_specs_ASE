#include <stdio.h>
#define Nmax 1500
#define Mmax 1500
#define INF 1000000

int main (void){
    int N, M;
    int i,j;
    int h[Nmax]={},w[Mmax]={};
    int counth[INF], countw[INF];
    int tmp;
    int sum;
 
    while(1){
        //?????????
        for(i=0;i<Nmax;i++)
            h[i]=0;
        for(i=0;i<Mmax;i++)
            w[i]=0;
        for(i=0;i<INF;i++)
            counth[i]=0;
        for(i=0;i<INF;i++)
            countw[i]=0;
        sum = 0;
        //??\???
        scanf("%d %d", &N, &M);
        if(!N && !M)
            break;
        for(i=0;i<N;i++)
            scanf("%d",&h[i]);
        for(i=0;i<M;i++)
            scanf("%d",&w[i]);
        //??????
        for(i=0;i<N;i++){
            tmp=0;
            for(j=i;j<N;j++){
                tmp += h[j];
                counth[tmp]++;
            }
        }
        for(i=0;i<M;i++){
            tmp=0;
            for(j=i;j<M;j++){
                tmp += w[j];
                countw[tmp]++;
            }
        }
        //??????
        for(i=0;i<INF;i++){
            sum += (counth[i] * countw[i]);
        }
        printf("%d\n",sum);
    }
    
    return 0;
}