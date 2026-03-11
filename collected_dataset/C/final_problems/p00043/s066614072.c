#include<stdio.h>

int pai[10];

int Check() {
	int insPai[10];
	int cnt;
	for (int head = 1; head < 10; head++) {
		for (int i = 0; i < 10; i++) {
			insPai[i] = pai[i];
		}
		cnt = 0;
		if (insPai[head] < 2)continue;
		insPai[head] -= 2;
		for (int shunt = 1; shunt < 8; shunt++) {
			if (insPai[shunt] <= 0)continue;
			else if (insPai[shunt + 1] > 0 && insPai[shunt + 2] > 0) {
				insPai[shunt]--; insPai[shunt + 1]--; insPai[shunt + 2]--;
				cnt++;
				shunt--;
			}
		}
		for (int ank = 1; ank < 10; ank++) {
			if (insPai[ank] < 3)continue;
			insPai[ank] -= 3;
			cnt++;
		}
		
		if (cnt == 4) return 1;
	}
	return 0;
}

int Check2() {
	int insPai[10];
	int cnt;
	for (int head = 1; head < 10; head++) {
		for (int i = 0; i < 10; i++) {
			insPai[i] = pai[i];
		}
		cnt = 0;
		if (insPai[head] < 2)continue;
		insPai[head] -= 2;
		for (int ank = 1; ank < 10; ank++) {
			if (insPai[ank] < 3)continue;
			insPai[ank] -= 3;
			cnt++;
		}
		for (int shunt = 1; shunt < 8; shunt++) {
			if (insPai[shunt] <= 0)continue;
			else if (insPai[shunt + 1] > 0 && insPai[shunt + 2] > 0) {
				insPai[shunt]--; insPai[shunt + 1]--; insPai[shunt + 2]--;
				cnt++;
				shunt--;
			}
		}
		if (cnt == 4) return 1;
	}
	return 0;
}

int main() {
	char input[20];
	int flag, flag2;
	int checker[9], pos;
	while (scanf("%s", input)==1) {
		pos = 0;
		for (int i = 0; i < 10; i++)pai[i] = 0;
		for (int i = 0; i < 13; i++) {
			pai[input[i] - 48]++;
		}
		for (int k = 1; k < 10; k++) {
			if (pai[k] == 4)continue;
			pai[k] += 1;
			flag = Check();
			flag2 = Check2();
			pai[k] --;
			if (flag||flag2) { checker[pos] = k; pos++; }
		}
		for (int i = 0; i < pos; i++) {
			printf("%d", checker[i]);
			if (i != pos - 1)printf(" ");
		}
		if (pos == 0)printf("0");
		printf("\n");
	}

	return 0;
}
