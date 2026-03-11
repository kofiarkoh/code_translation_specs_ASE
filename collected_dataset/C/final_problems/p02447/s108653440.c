#include <stdio.h>

typedef struct{
    int x,y;
}Z;

void output(Z z[],int n){
    int i;
    for(i=0;i<n;i++){
        printf("%d %d\n",z[i].x,z[i].y);
    }
    return;
}

void makeHeap(Z z[],int cnt,int i){
    if(i==0||i==1)return;
    
    int parent,l,r,min;
    parent=i/2;
    if(i%2==0){
        l=i;
        r=i+1;
    }else{
        l=i-1;
        r=i;
    }
    min=parent;
    
    if(z[l-1].x<z[min-1].x)min=l;
    else if(z[l-1].x==z[min-1].x){
        if(z[l-1].y<z[min-1].y)min=l;
    }
    
    if(r<=cnt&&z[r-1].x<z[min-1].x)min=r;
    else if(r<=cnt&&z[r-1].x==z[min-1].x){
        if(z[r-1].y<z[min-1].y)min=r;
    }
    
    if(min!=parent){
        Z tmp;
        tmp=z[min-1];
        z[min-1]=z[parent-1];
        z[parent-1]=tmp;
        
        makeHeap(z,cnt,parent);
    }
    
    return;
}
void reHeap(Z z[],int cnt,int parent){
    if(parent>cnt)return;
    
    int l,r,min;
    l=parent*2;
    r=parent*2+1;
    min=parent;
    
    if(l<=cnt&&z[l-1].x<z[min-1].x)min=l;
    else if(l<=cnt&&z[l-1].x==z[min-1].x){
        if(z[l-1].y<z[min-1].y)min=l;
    }
    
    if(r<=cnt&&z[r-1].x<z[min-1].x)min=r;
    else if(r<=cnt&&z[r-1].x==z[min-1].x){
        if(z[r-1].y<z[min-1].y)min=r;
    }
    
    if(min!=parent){
        Z tmp;
        tmp=z[min-1];
        z[min-1]=z[parent-1];
        z[parent-1]=tmp;
        
        reHeap(z,cnt,min);
    }
    return;
}

int main(void){
    int i;
    
    int n;
    scanf("%d\n",&n);
    Z z[n];
    int cnt=0;
    for(i=0;i<n;i++){
        scanf("%d %d\n",&z[i].x,&z[i].y);
        cnt++;
        makeHeap(z,cnt,cnt);
    }
    
    for(i=0;i<n;i++){
        printf("%d %d\n",z[0].x,z[0].y);
        z[0]=z[--cnt];
        reHeap(z,cnt,1);
    }
    
    return 0;
}

