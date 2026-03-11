#include <stdio.h>

int main (){
    long long int n;
    scanf ("%lld", &n);
    long long int arr[n];
    for (long long int i = 0; i < n; i++){
        scanf ("%lld", &arr[i]);
    }
    
    for (long long int i = 0; i < n; i++){
        if (arr[i] == 0){
            printf("0");
            return 0;
        }
    }
    
    long long int sum = 1;
    for (long long int i = 0; i < n; i++){
        if (arr[i] <= 1000000000000000000 / sum){
            sum *= arr[i];
        }
        else {    
        printf ("-1");
        return 0;
        }
    }
    
    printf ("%lld", sum);
    return 0;
}
