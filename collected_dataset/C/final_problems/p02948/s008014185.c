#include<stdio.h>
#include<stdlib.h>

typedef struct node{
    int a;
    int b;
}node;

int asc(const void* x, const void* y){
    return ((node*)x)->a - ((node*)y)->a;
}

void swap(node* x, node* y){
    if(x == y){
        return ;
    }
    node z = *x;
    *x = *y;
    *y = z;
}

void push(node* a, int* l, node b){
    *(a+*(l)) = b;
    *l += 1;
    int x = *l;
    while(x > 1){
        if((a+(x/2)-1)->b < (a+x-1)->b){
            //printf("%d %d <-> %d %d\n", (a+(x/2)-1)->a, (a+(x/2)-1)->b, (a+x-1)->a, (a+x-1)->b);
            swap(a+(x/2)-1, a+x-1);
        }
        x = x/2;
    }
}

node pop(node* a, int* l){
    node dst = *a;
    int x=1;
    swap(a, a+*(l)-1);
    *l -= 1;
    while(x <= (*(l)-1)/2){
        if((a+(2*x)-1)->b < (a+(2*x)+1-1)->b){
            if((a+x-1)->b < (a+(2*x)+1-1)->b){
                swap(a+x-1, a+(2*x)+1-1);
            }
            x = 2*x+1;
        }else{
            if((a+x-1)->b < (a+(2*x)-1)->b){
                swap(a+x-1, a+(2*x)-1);
            }
            x = 2*x;
        }
    }
    if(*l > 0){
        if((a+x-1)->b < (a+*(l)-1)->b){
            swap(a+x-1, a+*(l)-1);
        }
    }
    return dst;
}

int main(){
    int n, m;
    scanf("%d%d", &n, &m);
    node t[n];
    for(int i=0; i<n; i++){
        int a, b;
        scanf("%d%d", &a, &b);
        t[i].a = a;
        t[i].b = b;
    }
    qsort(t, n, sizeof(node), asc);
    /*printf("\n");
    for(int i=0; i<n; i++){
        printf("%d %d\n", t[i].a, t[i].b);
    }*/
    int l = 0, target = 0;
    node p[n];
    long long int ans = 0;
    for(int i=1; i<=m; i++){
        if(target < n){
            while(t[target].a <= i){
                /*printf("\n\n--before--\n");
                for(int i=0; i<l; i++){
                    printf("%d %d\n", p[i].a, p[i].b);
                }
                printf("--swap--\n");*/
                push(p, &l, t[target]);
                /*printf("--after--\n");
                for(int i=0; i<l; i++){
                    printf("%d %d\n", p[i].a, p[i].b);
                }*/
                target++;
                if(target == n){
                    break;
                }
            }
        }
        if(l > 0){
            ans += pop(p, &l).b;
        }
    }
    /*printf("\n");
    for(int i=0; i<l; i++){
        printf("%d %d\n", p[i].a, p[i].b);
    }*/
    printf("%lld", ans);

    return 0;
}