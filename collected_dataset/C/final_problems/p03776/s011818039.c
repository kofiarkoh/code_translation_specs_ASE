#include <stdio.h>

void merge(long long a[], int n, int left, int right);
double ave(long long a[], int n, int tail);
long long convination(int n, int i);

int main(void){
    int a, b, n;
    scanf("%d %d %d", &n, &a, &b);
    long long v[n];
    for(int i=0; i<n; i++){
        scanf("%lld", &v[i]);
    }
    //end to read

    merge(v, n, 0, n-1);  //sort has ended.

    double max=ave(v, n, a);

    int samen=0, samei=0;
    for(int l=0; l<n; l++){
        if(v[a-1]==v[l]){
            samen++;
            if(l<a){
                samei++;
            }
        }
    }

    unsigned long long count;

    count=convination(samen, samei);

    if((samei!=samen)&&(samei==a)){
        count =0;
        for(int m=a; m<=b&&m-a+1<=samen; m++){
            count += convination(samen, m);
        }
    }

    printf("%f\n%llu\n", max, count);
    return 0;
}

//---------------------------------------------------------------------main//

void merge(long long a[], int n, int left, int right){
    long long b[n];
    if(left==right){
        return;
    }
    else{
        int m=(right-left+1)/2 +left-1;
        merge(a, n, left, m);
        merge(a, n, m+1, right);
        int l=left, r=m+1, p=left;
        while(l<=m && r<=right){
            if(a[l]<a[r]){
                b[p]=a[r];
                r++;
                p++;
            }
            else{
                b[p]=a[l];
                l++;
                p++;
            }
        }
        while(l<=m){
            b[p]=a[l];
            l++;
            p++;
        }
        while(r<=right){
            b[p]=a[r];
            r++;
            p++;
        }
        for(int i=left; i<=right; i++){
            a[i]=b[i];
        }
    return;
    }
}

double ave(long long a[], int n, int tail){
    double sum=0;
    for(int i=0; i<tail; i++){
        sum += a[i];
    }
    double ans = sum/tail;
    return ans;
}

long long convination(int n, int i){
    long long t[n+1][n/2+1]; 
    for(int j=0; j<n+1; j++){
        for(int k=0; k<n+1 && k<n/2+1; k++){
            t[j][k]=0;
        }
    }
    t[0][0]=1; //set has ended

    for(int j=1; j<n+1; j++){
        t[j][0]=1;
        for(int k=1; k<j+1 && k<n/2+1; k++){
            t[j][k]=t[j-1][k-1]+t[j-1][k];
        }
    }

    long long ans;
    if(i>n/2){
        ans = t[n][n-i];
    }
    else{
        ans = t[n][i];
    }
    return ans;
}
