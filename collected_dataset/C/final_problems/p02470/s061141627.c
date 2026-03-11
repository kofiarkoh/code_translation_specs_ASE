#include <stdio.h>

int main(void) {
	int n;
	int asumi_kana[32];
	int kitamura_eri = 0;
	int horie_yui, tamura_yukari;
	int result;
	if (scanf("%d", &n) != 1) return 1;

	horie_yui = n;
	for (tamura_yukari = 2; tamura_yukari * tamura_yukari <= horie_yui; tamura_yukari++) {
		if (horie_yui % tamura_yukari == 0) {
			asumi_kana[kitamura_eri++] = tamura_yukari;
			while (horie_yui % tamura_yukari == 0) horie_yui /= tamura_yukari;
		}
	}
	if (horie_yui > 1) asumi_kana[kitamura_eri++] = horie_yui;

	result = n;
	tamura_yukari = 1 << kitamura_eri;
	for (horie_yui = 1; horie_yui < tamura_yukari; horie_yui++) {
		int s = 1, c = 1;
		int i;
		for (i = 0; i < tamura_yukari; i++) {
			if ((horie_yui >> i) & 1) {
				s = -s;
				c *= asumi_kana[i];
			}
		}
		result += s * (n / c);
	}
	printf("%d\n", result);
	return 0;
}

