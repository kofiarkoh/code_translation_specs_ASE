#include <stdio.h>

int main(int argc, const char * argv[]) {
    // insert code here...
    
    long int t = 0;
    long int n = 0;
    long int h = 0;
    long int k = 0;
    long int tmpt = 0;
    long int tmpn = 0;
    long int tmph = 0;
    long int result = 0;
    long int resultA = 0;
    
    scanf("%ld %ld %ld %ld",&t,&n,&h,&k);
    
//    if(t == n && n == h && t == h){
//        printf("0\n");
//        return 0;
//    }
    
    while (1) {
        if (k == 1 || k == 0) {
            break;
        }
        k = k / 2;
//        printf("%ld\n",k);
    }
    
    for(int i = 0;i < k;i++){
        tmpt = n + h;
        tmpn = t + h;
        tmph = t + n;
        t = tmpt;
        n = tmpn;
        h = tmph;
    }
    
    //1400 500 1100
    //1600 2500 1900
    
    result = t - n;
    
    //プラスに反転させる処理
    if(result < 0){
        resultA = result - result - result;
    }
    
    long int value = 1000000000000000000;
    
    if(resultA > value){
        printf("Unfair");
    }else{
        printf("%ld\n",result);
    }
    
    return 0;
}