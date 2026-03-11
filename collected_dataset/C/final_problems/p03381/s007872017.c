
#include<stdio.h>
#include<string.h>

void sort(int data[], int n){
    int temp;
    for(int i = 0; i < n; i++){
        for(int j = i + 1; j < n; j++){
            if(data[i] < data[j]){
                temp = data[i];
                data[i] = data[j];
                data[j] = temp;
            }
        }
    }
}

int main(void){
    int n;
    scanf("%d", &n);
    int data1[n], data2[n];
    for(int i = 0; i < n; i++){
        scanf("%d", &data1[i]);
    }
    memcpy(data2, data1, sizeof(data1));
    sort(data2, n);
    for(int i = 0; i < n; i++){
        for(int j = 0; j < n; j++){
            if(data1[i] == data2[j] && j <= n / 2){
                printf("%d\n", data2[n / 2]);
            }else{
                printf("%d\n", data2[n / 2 - 1]);
            }
            break;
        }
    }
    return 0;
}