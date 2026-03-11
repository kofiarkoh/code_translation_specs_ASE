#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int data[100] = {0};

int main()
{
	int n, m;
	int val, val2, ans;
	int temp;
	scanf("%d %d", &n, &m);
	val = m/n;
	val2 = val + (m%n);

	if (val == val2) {
		ans = val;
	} else {
		int limit = 0;
		if (val > val2) {
			limit = val2;
		} else {
			limit = val;
		}
		temp = limit;
		while(temp > 0) {
			if ((val%temp == 0) && (val2%temp == 0)) {
				ans = temp;
				break;
			} else {
				temp--;
			}
		}
	}

	printf("%d\n",ans);
	return 0;
}
