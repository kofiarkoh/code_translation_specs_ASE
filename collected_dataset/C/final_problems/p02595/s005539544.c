#include <stdio.h>
#include <math.h>
 
int main(void){
    long int N,D; //Nは点の個数　Dは原点からの距離
    long int x[100],y[100]; //X座標とY座標
    //NとDの読み込み
    scanf("%ld %ld", &N, &D);
    int i;
    //Nの数だけ　座標を打ち込む
    for(i=0; i<N; i++){
        scanf("%ld %ld", &x[i], &y[i]);
    }
    double r[100];
    //Nの数だけ、原点からの距離を読み込む
    for(i=0;i<N;i++){
        r[i] = sqrt(x[i]*x[i] + y[i]*y[i]);
    }
    int count = 0;
    //D以下をカウント
    for(i=0;i<N;i++){
        if(r[i] <= D){
            count++;
        }
    }
    printf("%d\n",count);
    return 0;    
}