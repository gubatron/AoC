package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day21 {
    // Try to intersect ingredient lists to find unique ingredients
    // when found, put in definite_allergen_ingredient map
    static int findDefiniteAllergenIngredient(
            HashMap<String, List<List<String>>> allergens_candidate_ingredients,
            HashMap<String, String> definite_allergen_ingredient) {
        allergens_candidate_ingredients.forEach((allergen, ingredient_lists) -> {
            Set<String> filter = new HashSet<>(ingredient_lists.get(0));
            ingredient_lists.stream().skip(1).forEach(filter::retainAll);
            if (filter.size() == 1) {
                definite_allergen_ingredient.put(allergen, filter.stream().findFirst().get());
            }
        });
        return definite_allergen_ingredient.size();
    }

    static HashMap<String, String> findAllergens(HashMap<String, List<List<String>>> allergens_candidate_ingredients) {
        final int totalAllergens = allergens_candidate_ingredients.keySet().size();
        // Keep the ones we know appear only once on the intersection of all candidate lists
        HashMap<String, String> definite_allergen_ingredient = new HashMap<>();

        // Keep finding definite allergen maps until we have the total number of allergens on the map
        while (findDefiniteAllergenIngredient(allergens_candidate_ingredients, definite_allergen_ingredient) < totalAllergens) {
            // remove definite allergen candidate lists
            definite_allergen_ingredient.forEach((def_allergen, def_ingredient) -> {
                allergens_candidate_ingredients.remove(def_allergen);

                HashMap<String, List<List<String>>> new_allergens_candidate_ingredients = new HashMap<>();
                allergens_candidate_ingredients.forEach(
                        (other_aller, ing_lists) ->
                                ing_lists.forEach(ing_list -> {
                                            final List<String> cleanedUpList = ing_list.stream().
                                                    filter(ing -> !ing.equals(def_ingredient)).
                                                    collect(Collectors.toList());
                                            if (!new_allergens_candidate_ingredients.containsKey(other_aller)) {
                                                new_allergens_candidate_ingredients.put(other_aller, new ArrayList<>());
                                            }
                                            new_allergens_candidate_ingredients.get(other_aller).add(cleanedUpList);
                                        }
                                ));
                allergens_candidate_ingredients.clear();
                allergens_candidate_ingredients.putAll(new_allergens_candidate_ingredients);
            });
        }
        return definite_allergen_ingredient;
    }

    public static long part1(HashMap<String, String> definite_allergen_ingredient,
                             List<List<String>> original_ingredient_lists) {
        // Now count all the words on each original ingredientlist that are not allergenic ingredients.
        List<String> safeIngredients = new ArrayList<>();
        original_ingredient_lists.forEach(ingredient_list -> ingredient_list.stream().filter(ingredient -> !definite_allergen_ingredient.containsValue(ingredient)).forEach(safeIngredients::add));
        return safeIngredients.size();
    }

    public static String part2(HashMap<String, String> definite_allergen_ingredient) {
        List<String> sorted_allergens = definite_allergen_ingredient.keySet().stream().sorted().collect(Collectors.toList());
        String ingredients = sorted_allergens.stream().map(allergen -> definite_allergen_ingredient.get(allergen) + ",").reduce(String::concat).get();
        return ingredients.substring(0, ingredients.length() - 1);
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_21.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_21.txt"), "\n");

        HashMap<String, List<List<String>>> allergens_candidate_ingredients = new HashMap<>();
        List<List<String>> original_ingredient_lists = new ArrayList<>();

        lines.forEach(line -> {
            String[] split = line.split(" \\(contains ");
            String[] ingredients = split[0].split(" ");
            String[] allergens = split[1].substring(0, split[1].length() - 1).split(", ");
            List<String> ingredientList = Arrays.asList(ingredients);
            original_ingredient_lists.add(ingredientList);
            // Create a map of ALLERGEN -> [ [ Ingredient List], [Ingredient List] ]
            Arrays.stream(allergens).forEach(allergen ->
            {
                if (allergens_candidate_ingredients.containsKey(allergen)) {
                    allergens_candidate_ingredients.get(allergen).add(ingredientList);
                } else {
                    List<List<String>> ingredients_lists = new ArrayList<>();
                    ingredients_lists.add(ingredientList);
                    allergens_candidate_ingredients.put(allergen, ingredients_lists);
                }
            });
        });

        System.out.println("DAY 21 - Allergen Assessment");
        long t_a = System.currentTimeMillis();
        HashMap<String, String> definite_allergen_ingredient = findAllergens(allergens_candidate_ingredients);
        System.out.println("Part 1: " + part1(definite_allergen_ingredient, original_ingredient_lists)); // 2230
        System.out.printf("%d ms\n", System.currentTimeMillis() - t_a);
        System.out.println("==============================");
        long t_b = System.currentTimeMillis();
        System.out.println("Part 2: " + part2(definite_allergen_ingredient)); // qqskn,ccvnlbp,tcm,jnqcd,qjqb,xjqd,xhzr,cjxv
        System.out.printf("%d ms\n", System.currentTimeMillis() - t_b);
    }
}
