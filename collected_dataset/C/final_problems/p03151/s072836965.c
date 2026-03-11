#include<stdio.h>
#include<stdlib.h>

#define NEW(p,n){p=malloc((n)*sizeof(p[0]));if(p==NULL){printf("not enough memory\n");exit(1);};}
//pの型の変数n個の要素分のメモリを確保し、そのアドレスをpに代入するマクロ

#define INFTY 999999

int main(void){
    int N;
    scanf("%d", &N);

    long* A;
    NEW(A,N);
    long* B;
    NEW(B,N);

    for(int i=0;i<N;i++){
        scanf("%ld", &A[i]);
    }
    for(int i=0;i<N;i++){
        scanf("%ld", &B[i]);
    }

    //A-Bが負になるもの(勉強足りない)ものの差の総和を格納する
    long long D=0;
    int d=0; //Dの要素数

    //A-Bが正になるもの(足りてる)を格納する
    long* P;
    NEW(P,N);
    int p=0; //Pの要素数

    for(int i=0;i<N;i++){
        if(A[i]<B[i]){
            D+=(-A[i]+B[i]);
            d++;
        }
        else if(A[i]>B[i]){
            P[p]=A[i]-B[i];
            p++;
        }
    }

    long max;
    int index;

    int ans=d;

    while(1){
        max=0;
        //Pの中で一番大きいP[i]を探す
        for(int i=0;i<p;i++){
            if(max<P[i]){
                max=P[i];
                index=i;
            }
        }
        if(max>0){
            if(D<=0){
                break;
            }
            D-=max;
            P[index]=0;
            ans++;
        }
        else{
            break;
        }
    }

    if(D<=0){
        printf("%d\n",ans);
    }
    else{
        printf("-1\n");
    }
    
    return 0;
    
}
