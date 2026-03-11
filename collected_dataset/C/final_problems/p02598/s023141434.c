#include <stdio.h>
#include <stdlib.h>

#define ll long long

int possible(int n, int k, int a[], int l){
    int pieces = 0;
    for(int i=0;i<n;i++){
        pieces += a[i]/l;
        if(a[i]%l==0) pieces -= 1;
    }
    if(pieces<=k) return 1;
    else return 0;
}

int main(){
    int n, k, a[200000];
    scanf("%d%d", &n, &k);
    for(int i=0;i<n;i++) scanf("%d", a+i);
    
    int min = 0, max = (int)1e9 + 1;
    while(max-min>1){
        int middle = (max+min)/2;
        if(possible(n, k, a, middle)){
            max = middle;
        }else{
            min = middle;
        }
    }
    
    printf("%d\n", max);

    return 0;
}