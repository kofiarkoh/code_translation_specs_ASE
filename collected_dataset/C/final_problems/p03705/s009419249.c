#include <stdio.h>

int c(int m,int n){
    int result = 1;
    int i = 0;
        if(n == 1 || m == n-1){
            return m;
        }else if(n ==0){
            return 0;
        }else if(n ==m){
            return 1;
        }
         else{
            for(i = n;i<=m;i++){
                result *= i;
            }
            for(i = 1;i<=n;i++){
                result /= i;
            }
        }

    return result;
}

int main()
{
    int sum = 0;
    int num1 = 0;
    int num2 = 0;

    scanf("%d%d%d",&sum,&num1,&num2);

    if(num1 == num2){
        if(sum > 1){
            printf("1");
        }
        else if (sum == 1){
            printf("1");
        }else{
            printf("0");
        }
    }else if(num1 > num2){
        printf("0");
    }else{
        if(sum == 1){
            printf("0");
        }else if (sum >=2){
            printf("%d",2+c(num2-num1+1,sum-2));
        }

    }

    return 0;
}
