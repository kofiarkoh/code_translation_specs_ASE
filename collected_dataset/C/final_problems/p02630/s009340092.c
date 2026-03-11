#include<stdio.h>
#include<math.h>
#include<string.h>
#include<stdlib.h>
#include<limits.h>
#define MOD 1000000007
#define PI 3.14159265
#define seive_len 1

typedef long long int lli;

int *array;
int seive[seive_len];

long long int min(long long int a, long long int b) {
    return a<b?a:b;
}

long long int max(long long int a, long long int b) {
    return a>b?a:b;
}

int compare(const void* a, const void* b) {
    return *(int*)a-*(int*)b;
}

int cmp(const void *a, const void *b){
    int ia = *(int *)a;
    int ib = *(int *)b;
    return array[ia] < array[ib] ? -1 : array[ia] > array[ib];
}

long double abs1(long double n) {
    if(n < 0)
        return n*(-1.0);
    return n*1.0;
}

void swap(int *a, int *b) {
    *a = (*a)^(*b);
    *b = (*a)^(*b);
    *a = (*a)^(*b);
}

lli binSearch(lli a[], lli x, lli l, lli r) {
    if(r >= l) {
        lli mid = l + ((r-l)>>1);
        if(a[mid] == x)
            return mid;
        if(a[mid] > x) {
            return binSearch(a, x, l, mid-1);
        }
        else if(a[mid] < x) {
            return binSearch(a, x, mid+1, r);
        }
    }
    return -1;
}

void makeSeive() {
    int i;
	seive[1] = 1;
    for(i=2;i<seive_len;i++) {
        if(seive[i] == 0) {
            long long int temp=(long long int)i*i;
            while(temp <= seive_len-1) {
                seive[temp] = 1;
                temp += i;
            }
        }
    }
}

int isPrime(int n) {
    int i;
    for(i=2;i*i<=n;i++)  {
        if(n%i == 0)
            return 0;
    }
    return 1;
}

void getInput(lli a[], lli n) {
    for(lli i=0;i<n;i++)
        scanf("%lld", &a[i]);
}

int titleToNumber(char* s)
{
    int sum = 0;
    while(*s) sum = 26*sum+(*(s++)-'A'+1);
    return sum;
}

void testCase() {
    lli n, i, k;
    scanf("%lld", &n);
    lli a[n];
    getInput(a, n);
    scanf("%lld", &k);
    lli sum = 0;
    for(i=0;i<n;i++)
        sum += a[i];
    // qsort(a, n, sizeof(lli), compare);
    lli b, c;
    lli freq[100001];
    for(i=0;i<100001;i++)
        freq[i] = 0;
    for(i=0;i<n;i++)
        freq[a[i]]++;
    for(i=0;i<k;i++) {
        scanf("%lld%lld", &b, &c);
        sum += (freq[b]*(c-b));
        freq[c] += freq[b];
        freq[b] = 0;
        printf("%lld\n", sum);
    }

}

int main() {
    int t=1;
    // scanf("%d", &t);
    // makeSeive();
    while(t--) {
        testCase();
        // printf("\n");
    }
    return 0;
}
