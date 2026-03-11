#include<stdio.h>
#include<string.h>

char array1[1001],array2[1001];
int a,b,array[1001][1001];
int min(int,int,int);
void search(int,int);

int main(void){
    int i,j;
    scanf("%s",array1);
    scanf("%s",array2);
    a = strlen(array1);
    b = strlen(array2);
    for(i = a - 1;i >= 0 ;i--){
        array1[i + 1] = array1[i];
    }
    array1[0] = '0';
    for(i = b - 1;i >= 0 ;i--){
        array2[i + 1] = array2[i];
    }
    array2[0] = '0';
    array[0][0] = 0;
    for(i = 0;i < a + 1;i++){
        array[i][0] = i;
    }
    for(j = 0;j < b + 1;j++){
        array[0][j] = j;
    }
    for(i = 1;i < a + 1;i++){
        for(j = 1;j < b + 1;j++){
            search(i,j);
        }
    }
    printf("%d\n",array[a][b]);
    return 0;
}

int min(int p,int q,int r){
    if(p <= q && p <= r){
        return p;
    }else if(q <= p && q <= r){
        return q;
    }else{
        return r;
    }
}

void search(int p,int q){
    if(array1[p] == array2[q]){
        array[p][q] = array[p - 1][q - 1];
    }else{
        array[p][q] = min(array[p - 1][q],array[p][q - 1],array[p - 1][q - 1]) + 1;
    }
}