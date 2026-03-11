#include <stdio.h>
#include <stdbool.h>
long long MOD=1e9+7;
int popcount(int x){
    int num  = 0;
    for (int mask=1; mask <(1<<30) ; mask = mask << 1 ){
        if (x & mask )
            num++;
    }

    return num;
}
int main(void){
    // Your code here!
    int N;
    char X[200001];
    scanf("%d %s",&N,X);
    int Xbit_num=0;
    for(int i=0;i<N;i++){
        if(X[i]=='1'){
            Xbit_num++;
        }
    }
    bool bit_one=false;
    long long minusnum=Xbit_num-1;
    long long plusnum=Xbit_num+1;
    if(Xbit_num==0){
        for(int i=0;i<N;i++){
            printf("%d\n",1);
        }
        return 0;
    }
    if(Xbit_num==1){
        bit_one=true;
        minusnum=2;
    }
    long long minussum=0;
    long long plussum=0;
    //printf("%d %d \n",minusnum,plusnum);
    //printf("%d %d \n",minussum,plussum);
    long long pow_index_minus[N+1];
    long long pow_index_plus[N+1];
    for(int i=0;i<N;i++){
        if(i==0){
            pow_index_minus[0]=1;
            pow_index_plus[0]=1;
            continue;
        }
        pow_index_minus[i]=pow_index_minus[i-1]*2%minusnum;
        pow_index_plus[i]=pow_index_plus[i-1]*2%plusnum;
        //printf("%d\n",pow_index_plus[i]);
    }
    for(int i=N-1;i>=0;i--){
        int index=N-i-1;
        //printf("%d\n",index);
        minussum+=pow_index_minus[i]*(X[index]-'0');
        plussum+=pow_index_plus[i]*(X[index]-'0');
        minussum%=minusnum;
        plussum%=plusnum;
       // printf("plussum %d %d %d\n", plussum,plusnum);
    }
    for(int i=0;i<N;i++){
        if(X[i]=='1'&&bit_one){
            printf("%d\n",0);
            continue;
        }
        int finalsum=0;
        if(X[i]=='1'){
            finalsum=(minussum+minusnum*300000-pow_index_minus[N-i-1])%minusnum;
        }else{
            finalsum=(plussum+pow_index_plus[N-i-1])%plusnum;
        }
        //printf("%d\n",pow_index_minus[N-i-1]);
        //printf("sum1 %d\n",finalsum);
        int res=1;
        while(finalsum!=0){
            finalsum=finalsum%popcount(finalsum);
            res++;
        }
        printf("%d\n",res);
    }
}
