SELECT registered.update_material_rating(
	v_material_id := $1, 
	v_user_id := $2, 
	v_rating := $3
);