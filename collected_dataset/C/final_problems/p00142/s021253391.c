#include <stdio.h>
#include <string.h>

int main(void)
{
    int i, j;
    int n;
    int list[5000];
    int bucket[5000];
    char exist[10001];
    int num;
    int temp;
    
    while (scanf("%d", &n) && n){
        num = 0;
        memset(exist, 0, sizeof(exist));
        memset(bucket, 0, sizeof(bucket));
        
        for (i = 1; i < n; i++){
            temp = i * i % n;
            if (!exist[temp]){
                exist[temp] = 1;
                list[num++] = temp;
            }
        }
        
        for (i = 0; i < num; i++){
            for (j = 0; j < num; j++){
                if (i == j){
                    continue;
                }
                temp = list[i] - list[j];
                if (temp < 0){
                    temp += n;
                }
                if (temp > (n - 1) / 2){
                    temp = n - temp;
                }
                bucket[temp]++;
            }
        }
        
        for (i = 1; i <= (n - 1) / 2; i++){
            printf("%d\n", bucket[i]);
        }
    }
    
    return (0);
}