#include<stdio.h>
#include<stdlib.h>

void swap(long long int* a,long long int* b){long long int c=*a;*a=*b;*b=c;}
int compare_ascending(const void* a, const void* b){
    long long int *A = (long long int *)a;
  long long int *B = (long long int *)b;
  if (*A > *B) return 1;
  if (*A < *B) return -1;
  return 0;
}

int main(){
    int n;
    scanf("%d", &n);
    long long int a[n] , b[n];
    long long int a2[n], b2[n];
    for(int i=0; i<n; i++){
        scanf("%lld", &a[i]);
        a2[i] = a[i];
    }
    for(int i=0; i<n; i++){
        scanf("%lld", &b[i]);
        b2[i] = b[i];
    }
    qsort(a2, n, sizeof(long long int), compare_ascending);
    qsort(b2, n, sizeof(long long int), compare_ascending);
    int id[n];
    int f = 1, in = 0;
    for(int i=0; i<n; i++){
        if(a2[i] > b2[i]){
            f = 0;
            break;
        }
    }
    long long int x = 0;
    for(int i=0; i<n; i++){
        int l = 0, r = n-1;
        int p = (l+r)/2;
        while(l < r){
            if(a2[p] == a[i]){
                break;
            }else if(a2[p] < a[i]){
                l = p+1;
            }else{
                r = p-1;
            }
            p = (l+r)/2;
        }
        if(b[i] != b2[p]){
            x++;
        }
    }
    if((x+1)/2 > n-2){
        f = 0;
    }
    if(f){
        printf("Yes");
    }else{
        printf("No");
    }


    return 0;
}