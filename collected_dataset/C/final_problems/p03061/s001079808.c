#include <stdio.h>
long func(long a,long b);
int main(){
    long N;
    long A[100000];
    long ret=-1;
    long i,j,z;
    long max,tmp,maxtmp;
    scanf("%d",&N);
    for(i=0;i<N;i++){
        scanf("%ld",&A[i]);
    }
    for(i=0;i<N;i++){
        maxtmp = A[0];
        for(j=0;j<N;j++){
            //i番目を書き換える
            if(j==i){
                if(i==0)maxtmp=A[1];
                //printf("cont\n");
                continue;
            }else{
                maxtmp = func(maxtmp,A[j]);
            }
            //printf("deb:%ld \n",maxtmp);
        }
        if(maxtmp>ret)ret=maxtmp;
    }
    printf("%ld\n",ret);
    return 0;
}

//最大公約数を求める
long func(long x,long y){
    long ret;
    long a,b;
    if(x>y){
        a = x;
        b = y;
    }else{
        a = y;
        b = x;
    }
    ret = a % b;
    while(ret!=0){
        a = b;
        b = ret;
        ret = a % b;
    }

    return b;
}