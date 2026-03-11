#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <float.h>

long double combination(long long all,long long cont);

long double memo[4000][4000];

int main(void){
    long long N;
    long long **meatveg;
    long long meatnum,vegnum;
    long long i,j;
    long double count=0;
    
    for (i=0;i<4000;i++){
        for (j=0;j<4000;j++){
            if(j==1){
                memo[i][j]=i;
            }
            else if (i==j){
                memo[i][j]=1;
            }
            else{
                memo[i][j]=0;
            }
        }
    }
    
    scanf("%lld",&N);
    meatveg=malloc(sizeof(long long*)*N);
    for (i=0;i<N;i++){
        meatveg[i]=malloc(sizeof(long long)*2);
    }
    for (i=0;i<N;i++){
        scanf("%lld %lld",&meatveg[i][0],&meatveg[i][1]);
    }

    for(i=0;i<N-1;i++){
        for(j=i+1;j<N;j++){
            meatnum=meatveg[i][0]+meatveg[j][0];
            vegnum=meatveg[i][1]+meatveg[j][1];
            if (meatnum>vegnum) count+=fmodl(combination((long long)meatnum+vegnum,(long long)vegnum),7+pow(10,9));
            else count+=fmodl(combination((long long)meatnum+vegnum,(long long)meatnum),7+pow(10,9));
        }
        
    }
    printf("%lld\n",(long long)fmodl(count,7+pow(10,9)));
    for (i=N-1;i>=0;i--){
        free(meatveg[i]);
    }
    free(meatveg);
    return 0;
}

long double combination(long long all,long long cont){
    if ((memo[all][cont])!=0) return memo[all][cont];
    else return memo[all][cont]=combination(all-1,cont)+combination(all-1,cont-1);
}