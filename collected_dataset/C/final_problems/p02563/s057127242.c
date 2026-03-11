#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdbool.h>

long modpow(long x, long y, long m){
	long res = 1l, tmp = x;
	while(y){
		if(y&1)res = res * tmp % m;
		tmp = tmp * tmp % m;
		y >>= 1;
	}
	return res;
}

int bit_rev(int x, int h){
	int res = 0;
	for(int i=0; i<h; i++){
		res <<= 1;
		res |= x&1;
		x >>= 1;
	}
	return res;
}
void bit_rev_perm(long a[], int h){
	long *A = (long*)malloc(sizeof(long) * (1<<h));
	memcpy(A, a, sizeof(long) * (1<<h));
	for(int i=0; i<1<<h; i++){
		a[bit_rev(i, h)] = A[i];
	}
	free(A);
	return;
}
void dft(long a[], int h, bool sign, long m, long r){
	bit_rev_perm(a, h);
	for(int s=1; s<=h; s++){
		int tmp = 1<<s;
		long zeta = modpow(r, (m-1)>>s, m);
		if(!sign)zeta = modpow(zeta, m-2, m);
		for(int k=0; k<1<<h; k+=tmp){
			long pow_zeta = 1;
			for(int j=0; j<tmp/2; j++){
				long t = pow_zeta * a[k+j+tmp/2] % m,
					 u = a[k+j];
				a[k+j] = (u + t) % m;
				a[k+j+tmp/2] = (m + u - t) % m;
				pow_zeta = zeta * pow_zeta % m;
			}
		}
	}
	return;
}

const long mod = 998244353;
int N, M;
long A[1<<20], B[1<<20];
int main(){
	scanf("%d%d", &N, &M);
	for(int i=0; i<N; i++)scanf("%ld", &A[i]);
	for(int i=0; i<M; i++)scanf("%ld", &B[i]);
	dft(A, 20, true, mod, 3);
	dft(B, 20, true, mod, 3);
	for(int i=0; i<1<<20; i++)A[i] = A[i] * B[i] % mod;
	dft(A, 20, false, mod, 3);
	long inv = modpow(1<<20, mod-2, mod);
	for(int i=0; i<N+M-1; i++){
		if(i)putchar(32);
		printf("%ld", A[i] * inv % mod);
	}putchar(10);
	return 0;
}