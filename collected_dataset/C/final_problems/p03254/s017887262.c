#include <stdio.h>
#include <stdlib.h>
int num(){
    char s[11];
    int i, n;
    for(i=0; i<sizeof(s); i++){
        s[i] = getchar();
        if(s[i] == ' ' || s[i] == '\n'){
            s[i] = '\0';
            break;
        }
    }
    sscanf(s, "%d", &n);
    return n;
}
void sort(int x[], int n){
    int y, i, j;
    for(i=n; i>1; i--){
        for(j=0; j<=i-2; j++){
            if(x[j] > x[j+1]){
                y = x[j];
                x[j] = x[j+1];
                x[j+1] = y;
            }
        }
    }
}
void sort2(int x[], int n){
    int y, i, j;
    for(i=n; i>1; i--){
        for(j=0; j<=i-2; j++){
            if(x[j] < x[j+1]){
                y = x[j];
                x[j] = x[j+1];
                x[j+1] = y;
            }
        }
    }
}
int main(void){
    int n, x;
    int a[100];
    int i, y, cou=0;
    
    n = num();
    x = num();
    for(i=0; i<n; i++){
        a[i] = num();
    }
    
    sort(a, n);
    
    for(i=0; i<n; i++){
        if(x >= a[i]){
            x -= a[i];
            cou++;
        }
    }
    
    sort2(a, n);
    
    if(cou == n){
        y = 1;
    }
    else{
        y = 0;
    }
    i=0;
    while(x > 0){
//        x -= a[i];
        if(y==1){
            cou--;
        }
        x -= a[i];
        if(x < 0){
            x *= -1;
        }
        if(a[i] >= x){
            break;
        }
        
    }
//    if(x < 0){
//        cou++;
//    }
    
    printf("%d\n", cou);
    
    return 0;
    
}