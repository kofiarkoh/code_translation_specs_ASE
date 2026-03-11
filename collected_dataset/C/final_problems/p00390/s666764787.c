#include <stdio.h>
int main(){
	unsigned n;
	scanf("%u", &n);
	char w[2] = {0, 0};
	int a[12];
	unsigned wmin[2];
	for(unsigned i = 0; i < n; ++i) scanf("%d", a + i);
	for(unsigned i = 0; i < n; ++i){
		unsigned tmp;
		scanf("%u", &tmp);
		if(!w[a[i]] || wmin[a[i]] > tmp){
			w[a[i]] = 1;
			wmin[a[i]] = tmp;
		}
	}
	unsigned ans;
	printf("%u\n", w[0] && w[1] ? wmin[0] + wmin[1] : 0u);
}

