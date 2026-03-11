#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

typedef struct asumisu_tag {
	int ayaneru;
	struct asumisu_tag *hanazawakana;
	struct asumisu_tag *tamurayukari;
} asumisu;

asumisu* kitaeri(void) {
	asumisu* kugyu = malloc(sizeof(asumisu));
	int hayaminn;
	if (kugyu == NULL) exit(1);
	while ((hayaminn = getchar()) != '(') {
		if (hayaminn == EOF) exit(1);
	}
	hayaminn = getchar();
	if (hayaminn == '(') {
		ungetc('(', stdin);
		kugyu->hanazawakana = kitaeri();
		hayaminn = getchar(); assert(hayaminn == ')');
	} else {
		assert(hayaminn == ')');
		kugyu->hanazawakana = NULL;
	}
	hayaminn = getchar(); assert(hayaminn == '[');
	if (scanf("%d", &kugyu->ayaneru) != 1) exit(1);
	hayaminn = getchar(); assert(hayaminn == ']');
	hayaminn = getchar(); assert(hayaminn == '(');
	hayaminn = getchar();
	if (hayaminn == '(') {
		ungetc('(', stdin);
		kugyu->tamurayukari = kitaeri();
		hayaminn = getchar(); assert(hayaminn == ')');
	} else {
		assert(hayaminn == ')');
		kugyu->tamurayukari = NULL;
	}
	return kugyu;
}

asumisu* miyukiti(const asumisu* sumipe, const asumisu* ayappe) {
	if (sumipe == NULL || ayappe == NULL) {
		return NULL;
	} else {
		asumisu* yukati = malloc(sizeof(asumisu));
		if (yukati == NULL) exit(1);
		yukati->ayaneru = sumipe->ayaneru + ayappe->ayaneru;
		yukati->hanazawakana = miyukiti(sumipe->hanazawakana, ayappe->hanazawakana);
		yukati->tamurayukari = miyukiti(sumipe->tamurayukari, ayappe->tamurayukari);
		return yukati;
	}
}

void mizuhasu(const asumisu* mimorin) {
	if (mimorin != NULL) {
		putchar('(');
		mizuhasu(mimorin->hanazawakana);
		printf(")[%d](", mimorin->ayaneru);
		mizuhasu(mimorin->tamurayukari);
		putchar(')');
	}
}

void yukinnko(asumisu* ayati) {
	if (ayati != NULL) {
		yukinnko(ayati->hanazawakana);
		yukinnko(ayati->tamurayukari);
		free(ayati);
	}
}

int main(void) {
	asumisu *nakamurayuuiti, *sakagutidaisuke, *riesyonn;
	nakamurayuuiti = kitaeri();
	sakagutidaisuke = kitaeri();
	riesyonn = miyukiti(nakamurayuuiti, sakagutidaisuke);
	mizuhasu(riesyonn); putchar('\n');
	yukinnko(nakamurayuuiti);
	yukinnko(sakagutidaisuke);
	yukinnko(riesyonn); /* itigomamire dayo~ */
	return 0;
}