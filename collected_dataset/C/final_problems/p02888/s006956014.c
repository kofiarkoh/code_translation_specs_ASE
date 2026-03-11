#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int cmpint(const void* n1, const void* n2){
	if (*(int*)n1 > * (int*)n2) {
		return 1;//降順なら-1
	}
	else if (*(int*)n1 < *(int*)n2) {
		return -1;//降順なら1
	}
	else {
		return 0;
	}
}

int main() {
	int n,ans=0;
	int l[2000]={};
	scanf("%d", &n);
	for (int i = 0; i < n; i++) {
		scanf("%d", &l[i]);
	}
	for (int i = 0; i < n; i++) {
		for (int j = i + 1; j < n; j++) {
			for (int k = j + 1; k < n; k++) {
				if((i==j)||(i==k)||(j==k))
					continue;
				if((l[i]<l[j]+l[k])&&(l[j]<l[i]+l[k])&&(l[k]<l[i]+l[j]))
					ans++;
			}
		}
	}
	printf("%d\n", ans);

	return 0;
}
